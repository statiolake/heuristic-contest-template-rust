use anyhow::{anyhow, Result};
use quote::ToTokens;
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    mem::replace,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use std::{io::prelude::*, mem::take};
use syn::{
    parse_str,
    spanned::Spanned,
    token::{Brace, Colon2},
    visit_mut::{self, VisitMut},
    File, Ident, Item, ItemExternCrate, ItemMod, ItemUse, PathSegment, UseName, UsePath, UseTree,
    Visibility,
};

pub fn main(_args: &[String]) -> Result<()> {
    let file_path: PathBuf = vec!["driver", "src", "main.rs"].into_iter().collect();
    let parsed = expand(&file_path)?;

    let stream = parsed.to_token_stream();
    let formatted = format(&stream.to_string())?;
    println!("{}", formatted);

    Ok(())
}

fn fix_use_all(
    crate_ident: Option<&str>,
    external_crate_idents: &[String],
    file: &mut File,
) -> Result<()> {
    let crate_ident = match crate_ident {
        Some(ident) => ident,
        None => return Ok(()),
    };

    struct ItemUseVisitor<'a> {
        crate_ident: &'a str,
        external_crate_idents: &'a [String],
        error: Option<anyhow::Error>,
    }

    impl VisitMut for ItemUseVisitor<'_> {
        fn visit_item_use_mut(&mut self, i: &mut ItemUse) {
            let error = self.error.take();
            self.error =
                error.or_else(|| fix_use(self.crate_ident, self.external_crate_idents, i).err());
            visit_mut::visit_item_use_mut(self, i);
        }
    }

    let mut visitor = ItemUseVisitor {
        crate_ident,
        external_crate_idents,
        error: None,
    };
    visitor.visit_file_mut(file);

    if let Some(error) = visitor.error {
        return Err(error);
    }

    Ok(())
}

fn fix_use(crate_ident: &str, external_crate_idents: &[String], item: &mut ItemUse) -> Result<()> {
    let span = item.span();

    if let UseTree::Path(path) = &mut item.tree {
        if path.ident == "crate" {
            // crate::{child...} を crate::{crate_ident}::{child...} に変換する

            // まずは {ident}::dummy を作成
            let tree_ident = UseTree::Path(UsePath {
                ident: Ident::new(crate_ident, span),
                colon2_token: Colon2(span),
                // ダミーの tree を入れておく
                tree: Box::new(UseTree::Name(UseName {
                    ident: Ident::new("dummy", span),
                })),
            });

            // パスを crate::{child...} から crate::{ident}::dummy へ置き換え
            // ついでに {child...} を得る
            let tree_child = replace(&mut path.tree, Box::new(tree_ident));

            // dummy を {child...} に置き換えて crate::{ident}::{child...} とする
            let tree_crate_ident_dummy = match &mut *path.tree {
                UseTree::Path(p) => &mut p.tree,
                _ => panic!("failed to re-acquire UsePath"),
            };
            *tree_crate_ident_dummy = tree_child;
        } else if external_crate_idents.contains(&path.ident.to_string()) {
            // crate:: で始まっていないパスは逆に crate:: を付ける
            // (バンドル後は extern crate ではなくモジュールの一つでしかないので)
            //
            // まずは {ident}::{child...} を取り出す。
            let tree_ident = replace(
                &mut item.tree,
                // ダミーの tree に入れ替えておく
                UseTree::Name(UseName {
                    ident: Ident::new("dummy", span),
                }),
            );

            // {crate}::{ident}::{child...} を作成
            let tree_crate_ident = UseTree::Path(UsePath {
                ident: Ident::new("crate", span),
                colon2_token: Colon2(span),
                tree: Box::new(tree_ident),
            });

            item.tree = tree_crate_ident;
        }
    }

    Ok(())
}

fn fix_path_all(
    crate_ident: Option<&str>,
    external_crate_idents: &[String],
    file: &mut File,
) -> Result<()> {
    let crate_ident = match crate_ident {
        Some(ident) => ident,
        None => return Ok(()),
    };

    struct PathVisitor<'a> {
        crate_ident: &'a str,
        external_crate_idents: &'a [String],
        error: Option<anyhow::Error>,
    }

    impl VisitMut for PathVisitor<'_> {
        fn visit_path_mut(&mut self, i: &mut syn::Path) {
            let error = self.error.take();
            self.error =
                error.or_else(|| fix_path(self.crate_ident, self.external_crate_idents, i).err());
            visit_mut::visit_path_mut(self, i);
        }
    }

    let mut visitor = PathVisitor {
        crate_ident,
        external_crate_idents,
        error: None,
    };
    visitor.visit_file_mut(file);

    if let Some(error) = visitor.error {
        return Err(error);
    }

    Ok(())
}

fn fix_path(
    crate_ident: &str,
    external_crate_idents: &[String],
    path: &mut syn::Path,
) -> Result<()> {
    let span = path.span();
    if path.segments.len() < 2 {
        // :: がない単一の名前は変換しない (なんかやらかしそうなので)
        return Ok(());
    }

    let first = path.segments.first().unwrap();
    if first.ident == "crate" {
        // crate::{child...} を crate::{crate_ident}::{child...} に変換する
        let mut segments: VecDeque<_> = path.segments.iter().skip(1).cloned().collect();
        segments.push_front(PathSegment {
            ident: Ident::new(crate_ident, span),
            arguments: syn::PathArguments::None,
        });
        segments.push_front(first.clone());
        *path = syn::Path {
            leading_colon: None,
            segments: segments.into_iter().collect(),
        };
    } else if external_crate_idents.contains(&first.ident.to_string()) {
        let mut segments: VecDeque<_> = path.segments.iter().cloned().collect();
        segments.push_front(PathSegment {
            ident: Ident::new("crate", span),
            arguments: syn::PathArguments::None,
        });
        *path = syn::Path {
            leading_colon: None,
            segments: segments.into_iter().collect(),
        };
    }

    Ok(())
}

pub fn expand_mod_all(crate_ident: Option<&str>, file: &mut File, file_path: &Path) -> Result<()> {
    for item in &mut file.items {
        if let Item::Mod(module) = item {
            expand_mod(crate_ident, module, file_path)?;
        }
    }

    Ok(())
}

pub fn expand_mod(
    crate_ident: Option<&str>,
    module: &mut ItemMod,
    container_path: &Path,
) -> Result<()> {
    if module.content.is_some() {
        return Ok(());
    }

    let file_path = find_mod_file(module, container_path)
        .ok_or_else(|| anyhow!("failed to find module source: {}", module.ident))?;
    let parsed = expand_file_under_crate(crate_ident, &file_path)?;

    module.content = Some((Brace(module.semi.span()), parsed.items));
    module.semi = None;

    Ok(())
}

fn find_mod_file(module: &ItemMod, container_path: &Path) -> Option<PathBuf> {
    let mut file_path = container_path.parent()?.to_path_buf();

    // .../modname.rs
    file_path.push(module.ident.to_string() + ".rs");
    if file_path.exists() {
        return Some(file_path);
    }

    file_path.pop();

    // .../modname/mod.rs
    file_path.push(module.ident.to_string());
    file_path.push("mod.rs");
    if file_path.exists() {
        return Some(file_path);
    }

    None
}

fn extract_extern_crates(parsed: &mut File) -> Vec<ItemExternCrate> {
    let mut extern_crates = vec![];
    let mut items = vec![];
    for item in take(&mut parsed.items) {
        match item {
            Item::ExternCrate(extern_crate) => extern_crates.push(extern_crate),
            _ => items.push(item),
        }
    }
    parsed.items = items;

    extern_crates
}

fn expand_file_under_crate(crate_ident: Option<&str>, file_path: &Path) -> Result<File> {
    let source = read_to_string(file_path)?;
    let mut parsed: File = parse_str(&source)?;
    expand_mod_all(crate_ident, &mut parsed, file_path)?;

    Ok(parsed)
}

fn expand_crate(
    crate_ident: Option<&str>,
    file_path: &Path,
) -> Result<(File, Vec<ItemExternCrate>)> {
    let source = read_to_string(file_path)?;
    let mut parsed: File = parse_str(&source)?;
    let extern_crates = extract_extern_crates(&mut parsed);
    expand_mod_all(crate_ident, &mut parsed, file_path)?;

    let external_crate_idents = extern_crates
        .iter()
        .map(|krate| krate.ident.to_string())
        .collect::<Vec<_>>();
    fix_use_all(crate_ident, &external_crate_idents, &mut parsed)?;
    fix_path_all(crate_ident, &external_crate_idents, &mut parsed)?;

    Ok((parsed, extern_crates))
}

pub fn expand(file_path: &Path) -> Result<File> {
    let (mut main_crate, extern_crates) = expand_crate(None, file_path)?;

    let mut expanded_crates = HashMap::new();
    let mut queue = VecDeque::from(extern_crates);
    while let Some(krate) = queue.pop_front() {
        let crate_ident = krate.ident.to_string();
        if expanded_crates.contains_key(&crate_ident) {
            continue;
        }

        let path: PathBuf = vec![&*crate_ident.replace('_', "-"), "src", "lib.rs"]
            .into_iter()
            .collect();
        let (expanded, another_extern_crates) = expand_crate(Some(&crate_ident), &path)?;
        queue.extend(another_extern_crates);
        expanded_crates.insert(crate_ident, expanded);
    }

    let original_items = take(&mut main_crate.items);
    for (crate_ident, krate) in expanded_crates {
        let span = krate.span();
        let item = Item::Mod(ItemMod {
            attrs: vec![],
            vis: Visibility::Inherited,
            mod_token: syn::token::Mod { span },
            ident: Ident::new(&crate_ident, span),
            content: Some((Brace(span), krate.items)),
            semi: None,
        });

        main_crate.items.push(item);
    }
    main_crate.items.extend(original_items);

    Ok(main_crate)
}

pub fn format(source: &str) -> Result<String> {
    let mut proc = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = proc.stdin.take().unwrap();
    write!(stdin, "{}", source)?;
    drop(stdin);

    let output = proc.wait_with_output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
