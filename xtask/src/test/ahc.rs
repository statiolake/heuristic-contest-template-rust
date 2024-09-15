use anyhow::{bail, Context as _, Result};
use io::{source::Source, InitInput};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use solutions::get_solution_names;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, io::BufReader};
use std::{fs, hash::Hash};
use std::{
    fs::File,
    process::{Command, Stdio},
};
use std::{io::Write, time::Instant};

use crate::table::{Alignment, Table, TableCell};

static ABSOLUTE_BETTER: AbsoluteBetterIs = AbsoluteBetterIs::Maximum;

static SOLUTIONS: Lazy<Vec<Solution>> = Lazy::new(|| {
    get_solution_names()
        .into_iter()
        .map(Solution::new)
        .collect_vec()
});

static PRIMARY_SOLUTION: Lazy<Solution> = Lazy::new(|| SOLUTIONS.last().cloned().unwrap());

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cache {
    cache_path: PathBuf,
    seeds_txt_hash: Option<String>,
    results: HashMap<Solution, HashMap<Seed, TestCaseResult>>,
}

impl Cache {
    pub fn load_or_new(cache_path: &Path) -> Result<Self> {
        Self::load(cache_path).or_else(|_| Ok(Self::new(cache_path.to_owned())))
    }

    pub fn new(cache_path: PathBuf) -> Self {
        Self {
            cache_path,
            seeds_txt_hash: None,
            results: HashMap::new(),
        }
    }

    pub fn load(cache_path: &Path) -> Result<Self> {
        let file = File::open(cache_path).context("failed to open cached results file")?;
        serde_json::from_reader(file).context("failed to parse cached results")
    }

    pub fn save(&self) -> Result<()> {
        let file =
            File::create(&self.cache_path).context("failed to create cached results file")?;
        serde_json::to_writer(file, self).context("failed to serialize cached results into JSON")
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("failed to save cache: {e}");
        }
    }
}

pub fn main() -> Result<()> {
    let tester = Tester::detect().context("failed to detect testing tools")?;
    let cache_path = tester.testing_dir.join("cache.json");
    let mut cache = Cache::load_or_new(&cache_path)?;

    for solution in &*SOLUTIONS {
        if solution != &*PRIMARY_SOLUTION && cache.results.contains_key(solution) {
            eprintln!(
                "skipping non-primary and cached solution: {}",
                solution.inner()
            );
            continue;
        }

        eprintln!("running solution: {}", solution.inner());
        let env = TestEnvironment::new(&mut cache, tester.clone(), solution.clone())
            .context("failed to initialize test environment")?;
        let seeds = env.seeds.clone();
        if solution != &*PRIMARY_SOLUTION
            && cache.results.contains_key(solution)
            && cache.results[solution].keys().cloned().collect_vec() == seeds
        {
            continue;
        }

        cache.results.insert(
            solution.clone(),
            env.run_solution().context("failed to run solution")?,
        );
    }

    TablePrinter::new(cache.results.clone()).print();

    Ok(())
}

#[derive(Debug)]
// This enum is only used as a constant, so `dead_code` detection is not useful here.
#[allow(dead_code)]
enum AbsoluteBetterIs {
    Minimum,
    Maximum,
}

impl AbsoluteBetterIs {
    pub fn _is_former<T: Ord>(&self, a: T, b: T) -> bool {
        match self {
            Self::Minimum => a < b,
            Self::Maximum => a > b,
        }
    }

    fn is_which<T: Ord>(&self, values: impl Iterator<Item = T>) -> Option<T> {
        match self {
            Self::Minimum => values.min(),
            Self::Maximum => values.max(),
        }
    }

    fn can_be_sorted_by_this_key(&self, value: u64) -> i64 {
        match self {
            Self::Minimum => value as i64,
            Self::Maximum => -(value as i64),
        }
    }

    fn make_relative_goodness(&self, value: u64, best: u64) -> f64 {
        match self {
            Self::Minimum => best as f64 / value.max(1) as f64,
            Self::Maximum => value as f64 / best.max(1) as f64,
        }
    }

    fn is_always_better_than_this_value(&self) -> u64 {
        match self {
            Self::Minimum => 1e9 as u64,
            Self::Maximum => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCaseResult {
    seed: Seed,
    in_filename: String,
    init_input: InitInput,
    score: Result<u64, ()>,
    duration_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
struct Seed(String);

impl Seed {
    fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn inner(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
struct Solution(String);

impl Solution {
    fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    fn inner(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct Tester {
    testing_dir: PathBuf,
    bin_gen: PathBuf,
    bin_vis: PathBuf,
    bin_tester: Option<PathBuf>,
}

impl Tester {
    pub fn detect() -> Result<Self> {
        let testing_dir = PathBuf::from("testing");
        let testing_tools_dir = testing_dir.join("tools");

        let testing_binaries_dir =
            Self::ensure_built(&testing_tools_dir).context("failed to locate testing tools")?;
        let bin_gen = testing_binaries_dir.join("gen");
        let bin_vis = testing_binaries_dir.join("vis");
        let bin_tester = Some(testing_binaries_dir.join("tester")).filter(|p| p.exists());

        Ok(Self {
            testing_dir,
            bin_gen,
            bin_vis,
            bin_tester,
        })
    }

    fn ensure_built(testing_tools_dir: &Path) -> Result<PathBuf> {
        let testing_binaries_dir: PathBuf = testing_tools_dir.join("target").join("release");

        if testing_binaries_dir.exists() {
            return Ok(testing_binaries_dir);
        }

        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(testing_tools_dir)
            .output()
            .context("failed to build testing tools")?;

        Ok(testing_binaries_dir)
    }
}

#[derive(Debug)]
struct TestEnvironment {
    target_solution: Solution,
    tester: Tester,
    in_dir: PathBuf,
    out_dir: PathBuf,
    seeds: Vec<Seed>,
    in_filenames: Vec<String>,
}

impl TestEnvironment {
    fn new(cache: &mut Cache, tester: Tester, target_solution: Solution) -> Result<Self> {
        let in_dir = Path::new("testing").join("in");
        let out_dir = Path::new("testing")
            .join("out")
            .join(target_solution.inner());

        let seeds =
            Self::ensure_seeds(cache, &tester, &in_dir).context("failed to ensure seeds")?;
        let in_filenames: Vec<_> = fs::read_dir(&in_dir)
            .context("failed to read input directories")?
            .filter_map(Result::ok)
            .map(|entry| {
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| name.ends_with(".txt"))
            .collect();

        Ok(Self {
            target_solution,
            tester,
            in_dir,
            out_dir,
            seeds,
            in_filenames,
        })
    }

    fn ensure_seeds(cache: &mut Cache, tester: &Tester, in_dir: &Path) -> Result<Vec<Seed>> {
        let seeds_txt_contents = fs::read_to_string(tester.testing_dir.join("seeds.txt"))
            .context("failed to read seeds.txt")?;
        let seeds_txt_hash = sha256::digest(&seeds_txt_contents);
        let seeds: Vec<_> = seeds_txt_contents.lines().map(Seed::new).collect();

        if in_dir.exists() && cache.seeds_txt_hash.as_ref() == Some(&seeds_txt_hash) {
            eprintln!("seeds.txt is not changed; reusing seeds");
            return Ok(seeds);
        }

        cache.seeds_txt_hash = Some(seeds_txt_hash);
        eprintln!("regenerating seeds");

        if in_dir.exists() {
            fs::remove_dir_all(in_dir).context("failed to remove existing input directory")?;
        }

        // We need to run generator at the testing directory, so we need to get the relative path
        // to the binary from testing directory.
        let bin_gen_relative = tester
            .bin_gen
            .strip_prefix(&tester.testing_dir)
            .context("failed to get relative path for generator")?;
        Command::new(bin_gen_relative)
            .arg("seeds.txt")
            .current_dir(&tester.testing_dir)
            .output()
            .with_context(|| {
                format!(
                    "failed to execute generator at {}",
                    tester.bin_gen.display()
                )
            })?;

        Ok(seeds)
    }

    fn ensure_out_dir(&self) -> Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir)
                .context("failed to remove existing output directory")?;
        }
        fs::create_dir_all(&self.out_dir).context("failed to create output directory")?;

        Ok(())
    }

    fn run_solution(&self) -> Result<HashMap<Seed, TestCaseResult>> {
        self.ensure_out_dir()
            .context("failed to ensure output directory")?;

        eprintln!("building binary");
        let out = Command::new("cargo")
            .args(["build", "--release"])
            .spawn()
            .context("failed to execute solution binary")?
            .wait()?;

        if !out.success() {
            bail!("failed to build solution binary");
        }

        let results: HashMap<Seed, TestCaseResult> = (0..self.in_filenames.len())
            .into_par_iter()
            .map(|case_index| {
                self.test_for_input_index(case_index)
                    .map(|r| (r.seed.clone(), r))
                    .with_context(|| {
                        format!("failed to run test for {}", self.in_filenames[case_index])
                    })
            })
            .collect::<Result<_>>()
            .context("some test cases critically failed")?;

        Ok(results)
    }

    fn test_for_input_index(&self, case_index: usize) -> Result<TestCaseResult> {
        let in_filename = &self.in_filenames[case_index];
        let total_cases = self.in_filenames.len();

        println!(
            "testing {} by {}... ({}/{})",
            in_filename,
            self.target_solution.inner(),
            case_index + 1,
            total_cases
        );

        let number: usize = in_filename
            .trim_end_matches(".txt")
            .parse()
            .context("failed to parse input file name index")?;
        let seed = &self.seeds[number];

        let in_file_path = self.in_dir.join(in_filename);
        let out_file_path = self.out_dir.join(in_filename);
        let err_file_path = self.out_dir.join(format!("{}.stderr", in_filename));

        let in_file_content =
            fs::read_to_string(&in_file_path).context("failed to read input file contents")?;
        let init_input =
            InitInput::read_from(&mut Source::new(BufReader::new(in_file_content.as_bytes())));

        let start_time = Instant::now();

        let mut main_process = if let Some(bin_tester) = &self.tester.bin_tester {
            // Interactive
            Command::new(bin_tester)
                .arg(Path::new("target").join("release").join("main"))
                .arg(self.target_solution.inner())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("failed to spawn tester (interactive)")?
        } else {
            // Non-interactive
            Command::new(Path::new("target").join("release").join("main"))
                .env("RUST_BACKTRACE", "1")
                .arg(self.target_solution.inner())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("failed to spawn solution binary (non-interactive)")?
        };

        if let Some(mut stdin) = main_process.stdin.take() {
            stdin
                .write_all(in_file_content.as_bytes())
                .context("failed to write to stdin")?;
        }

        let output = main_process
            .wait_with_output()
            .context("failed to wait for main process to finish")?;
        let duration_millis = start_time.elapsed().as_millis() as i64;

        fs::write(&out_file_path, &output.stdout).context("failed to write stdout to file")?;
        fs::write(&err_file_path, &output.stderr).context("failed to write stderr to file")?;

        let output = Command::new(&self.tester.bin_vis)
            .args([&in_file_path, &out_file_path])
            .output()
            .context("failed to run visualizer")?;

        let vis_out_output = String::from_utf8_lossy(&output.stdout).into_owned()
            + &String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r"Score = (?<score>\d*)").unwrap();
        let score = re
            .captures(&vis_out_output)
            .and_then(|m| {
                m.name("score")
                    .expect("named capture not found")
                    .as_str()
                    .parse()
                    .ok()
            })
            .filter(|&score| score > 0)
            .ok_or(());

        // Append visualizer result
        // To keep output file valid, we need to append the result to the stderr file even though
        // tester does write to stdout.
        File::options()
            .append(true)
            .open(&err_file_path)
            .context("failed to open stdout file")?
            .write_all(vis_out_output.as_bytes())
            .context("failed to append visualizer stdout")?;

        // Append duration
        File::options()
            .append(true)
            .open(&err_file_path)
            .context("failed to open stdout file")?
            .write_all(format!("\nDuration: {} ms\n", duration_millis).as_bytes())
            .context("failed to append duration")?;

        Ok(TestCaseResult {
            in_filename: in_filename.clone(),
            seed: seed.clone(),
            score,
            init_input,
            duration_millis,
        })
    }
}

#[derive(Debug)]
struct TablePrinter {
    _solution_seed_results: HashMap<Solution, HashMap<Seed, TestCaseResult>>,
    seed_solution_results: HashMap<Seed, HashMap<Solution, TestCaseResult>>,
}

impl TablePrinter {
    fn new(solution_seed_results: HashMap<Solution, HashMap<Seed, TestCaseResult>>) -> Self {
        let seed_solution_results = Self::transpose_results(&solution_seed_results);

        Self {
            _solution_seed_results: solution_seed_results,
            seed_solution_results,
        }
    }

    fn print(&self) {
        let mut table = Table::new();

        self.render_header(&mut table);
        let (solution_total_absolute_score, solution_total_relative_score) =
            self.render_body(&mut table);
        self.render_footer(
            &mut table,
            &solution_total_absolute_score,
            &solution_total_relative_score,
        );

        table.print();
    }

    fn transpose_results(
        results: &HashMap<Solution, HashMap<Seed, TestCaseResult>>,
    ) -> HashMap<Seed, HashMap<Solution, TestCaseResult>> {
        let solutions = results.keys().cloned().collect_vec();
        let seeds = results[&*PRIMARY_SOLUTION].keys().cloned().collect_vec();

        // Transpose `Solution -> Seed -> Result` to `Seed -> Solution -> Result`
        seeds
            .iter()
            .map(|seed| {
                let result_of_seed: HashMap<_, _> = solutions
                    .iter()
                    .map(|solution| (solution.clone(), results[solution][seed].clone()))
                    .collect();

                (seed.clone(), result_of_seed)
            })
            .collect()
    }

    fn render_header(&self, table: &mut Table) {
        // Seed
        table.header.push(TableCell {
            content: "seed".to_string(),
            alignment: Alignment::Left,
        });

        // Parameters
        for key in InitInput::description_keys() {
            table.header.push(TableCell {
                content: key.to_string(),
                alignment: Alignment::Left,
            });
        }

        // Solutions
        for solution in &*SOLUTIONS {
            table.header.push(TableCell {
                content: solution.inner().to_string(),
                alignment: Alignment::Left,
            });
        }
    }

    fn render_body(&self, table: &mut Table) -> (HashMap<Solution, u64>, HashMap<Solution, f64>) {
        // Sort results of seeds by the score of the primary solution
        let sorted_results =
            self.seed_solution_results
                .iter()
                .sorted_by_key(|(_, solution_results)| {
                    solution_results[&*PRIMARY_SOLUTION]
                        .score
                        .map(|x| ABSOLUTE_BETTER.can_be_sorted_by_this_key(x))
                });

        let mut solution_total_absolute_score = SOLUTIONS
            .iter()
            .map(|s| (s.clone(), 0))
            .collect::<HashMap<_, _>>();
        let mut solution_total_relative_score = SOLUTIONS
            .iter()
            .map(|s| (s.clone(), 0.0))
            .collect::<HashMap<_, _>>();

        for (seed, solution_results) in sorted_results {
            let primary_result = &solution_results[&*PRIMARY_SOLUTION];

            let mut row = vec![];

            // Seed
            row.push(TableCell {
                content: seed.inner().to_string(),
                alignment: Alignment::Right,
            });

            // Parameters
            for value in primary_result.init_input.description_values() {
                row.push(TableCell {
                    content: value.clone(),
                    alignment: Alignment::Right,
                });
            }

            // Solutions
            let best_score = ABSOLUTE_BETTER
                .is_which(solution_results.values().flat_map(|r| r.score))
                .unwrap_or_else(|| ABSOLUTE_BETTER.is_always_better_than_this_value());
            for solution in &*SOLUTIONS {
                let result = &solution_results[solution];
                let absolute_score = result
                    .score
                    .unwrap_or_else(|_| ABSOLUTE_BETTER.is_always_better_than_this_value());
                let relative_score =
                    ABSOLUTE_BETTER.make_relative_goodness(absolute_score, best_score);
                let absolute_score_display = if result.score.is_ok() {
                    format!("{}", absolute_score)
                } else {
                    "ERROR".to_string()
                };
                row.push(TableCell {
                    content: format!("{:>10} / {:>10.8}", absolute_score_display, relative_score),
                    alignment: Alignment::Right,
                });
                *solution_total_absolute_score
                    .get_mut(solution)
                    .expect("unknown solution") += absolute_score;
                *solution_total_relative_score
                    .get_mut(solution)
                    .expect("unknown solution") += relative_score;
            }

            table.body.push(row);
        }

        (solution_total_absolute_score, solution_total_relative_score)
    }

    fn render_footer(
        &self,
        table: &mut Table,
        solution_total_absolute_score: &HashMap<Solution, u64>,
        solution_total_relative_score: &HashMap<Solution, f64>,
    ) {
        // Seed
        table.footer.push(TableCell {
            content: "total".to_string(),
            alignment: Alignment::Left,
        });

        // Parameters
        for _ in InitInput::description_keys() {
            table.footer.push(TableCell {
                content: "".to_string(),
                alignment: Alignment::Left,
            });
        }

        // Solutions
        for solution in &*SOLUTIONS {
            table.footer.push(TableCell {
                content: format!(
                    "{:>10} / {:>10.8}",
                    solution_total_absolute_score[solution],
                    solution_total_relative_score[solution]
                ),
                alignment: Alignment::Right,
            });
        }
    }
}
