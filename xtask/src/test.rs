use anyhow::Result;
use itertools::Itertools;

use self::multi_test::MultiTestConfig;

pub mod multi_test;

pub fn main(args: &[String]) -> Result<()> {
    // TODO: ゲームの種類によって呼び分けるようにする
    let mut cfg = MultiTestConfig::new();

    if !args.is_empty() {
        // 引数はすべて対象の解法名とする
        cfg = cfg.solution_names(args.to_vec());
    }

    cfg.run()
}
