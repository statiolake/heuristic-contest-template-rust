use anyhow::Result;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::SystemTime;
use std::{collections::HashMap, env::current_dir};

const SOLUTIONS: &[&str] = &["naive", "turn_base"];
const PRIMARY_SOLUTION: &str = SOLUTIONS[1];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestResult {
    seed: String,
    filename: String,
    score: i32,
    time: DateTime<Utc>,
    n: i32,
}

impl TestResult {
    fn desc(&self) -> String {
        format!(
            "{} (N = {}) ... score = {}",
            self.filename, self.n, self.score
        )
    }
}

struct TestEnvironment {
    solution: String,
    _bin_dir: PathBuf,
    _bin_gen: PathBuf,
    bin_vis: PathBuf,
    _bin_tester: PathBuf,
    in_dir: PathBuf,
    out_dir: PathBuf,
    seeds: Vec<String>,
    in_filenames: Vec<String>,
}

impl TestEnvironment {
    fn new(solution: String) -> Result<Self> {
        let script_dir = std::env::current_exe()?
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let bin_dir = script_dir.join("testing/tools/target/release");
        let bin_gen = bin_dir.join("gen");
        let bin_vis = bin_dir.join("vis");
        let bin_tester = bin_dir.join("tester");
        let in_dir = PathBuf::from("testing/in");
        let out_dir = PathBuf::from("testing/out").join(&solution);

        let seeds = Self::ensure_seeds(&bin_gen, &in_dir)?;
        let in_filenames: Vec<_> = fs::read_dir(&in_dir)?
            .filter_map(Result::ok)
            .map(|entry| {
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        Ok(Self {
            solution,
            _bin_dir: bin_dir,
            _bin_gen: bin_gen,
            bin_vis,
            _bin_tester: bin_tester,
            in_dir,
            out_dir,
            seeds,
            in_filenames,
        })
    }

    fn ensure_seeds(bin_gen: &Path, in_dir: &Path) -> Result<Vec<String>> {
        if in_dir.exists() {
            fs::remove_dir_all(in_dir)?;
        }

        Command::new(bin_gen)
            .arg("seeds.txt")
            .current_dir("testing")
            .output()?;

        let seeds: Vec<String> = fs::read_to_string("testing/seeds.txt")?
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(seeds)
    }

    fn ensure_out_dir(&self) -> Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir)?;
        }
        fs::create_dir_all(&self.out_dir)?;

        Ok(())
    }

    fn try_solution(&self) -> Result<HashMap<String, TestResult>> {
        std::env::set_current_dir(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )?;
        self.ensure_out_dir()?;
        Command::new("cargo")
            .args(["build", "--release"])
            .output()?;

        let results: Vec<TestResult> = (0..self.in_filenames.len())
            .into_par_iter()
            .map(|i| self.test_for_input_index(i).unwrap())
            .collect();

        Ok(results
            .into_iter()
            .map(|result| (result.seed.clone(), result))
            .collect())
    }

    fn test_for_input_index(&self, case_index: usize) -> Result<TestResult> {
        let in_filename = &self.in_filenames[case_index];
        let total_cases = self.in_filenames.len();

        println!(
            "testing {} by {}... ({}/{})",
            in_filename,
            self.solution,
            case_index + 1,
            total_cases
        );

        let no: usize = in_filename.trim_end_matches(".txt").parse().unwrap();
        let seed = &self.seeds[no];

        let in_file_path = self.in_dir.join(in_filename);
        let out_file_path = self.out_dir.join(in_filename);
        let err_file_path = self.out_dir.join(format!("{}.stderr", in_filename));

        assert!(
            in_file_path.exists(),
            "input file not found: {:?}, {:?}",
            current_dir(),
            in_file_path
        );

        let in_file_content = fs::read_to_string(&in_file_path)?;
        let mut in_file_lines = in_file_content.lines();
        let n: i32 = in_file_lines.next().unwrap().trim().parse().unwrap();

        let start_time = SystemTime::now();
        let mut main_process = Command::new("target/release/main")
            .arg(&self.solution)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = main_process.stdin.take() {
            stdin.write_all(in_file_content.as_bytes())?;
        }

        let output = main_process.wait_with_output()?;
        let _end_time = SystemTime::now();

        fs::write(&out_file_path, &output.stdout)?;
        fs::write(err_file_path, &output.stderr)?;

        Command::new(&self.bin_vis)
            .args([&in_file_path, &out_file_path])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let err_output = String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r"Score = (\d*)").unwrap();
        let score = if let Some(cap) = re.captures(&err_output) {
            cap[1].parse().unwrap_or(1_000_000_000)
        } else {
            1_000_000_000
        };

        Ok(TestResult {
            filename: in_filename.clone(),
            seed: seed.clone(),
            score,
            time: DateTime::<Utc>::from(start_time),
            n,
        })
    }
}

fn print_results(
    results: &HashMap<String, HashMap<String, TestResult>>,
    solutions: &[&str],
    primary_solution: &str,
) {
    let mut sorted_results: Vec<_> = results[primary_solution]
        .keys()
        .map(|seed| {
            let result_for_solution = solutions
                .iter()
                .map(|&solution| (solution.to_string(), results[solution][seed].clone()))
                .collect::<HashMap<_, _>>();
            (seed.clone(), result_for_solution)
        })
        .collect();

    sorted_results
        .sort_by_key(|(_, result_for_solution)| result_for_solution[primary_solution].score);

    let solution_width = 25;

    println!(
        "| seed |  N | {} |",
        solutions
            .iter()
            .map(|&solution| format!("{:^width$}", solution, width = solution_width))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!(
        "|------|----|-{}-|",
        solutions
            .iter()
            .map(|_| format!("{:-^width$}", "", width = solution_width))
            .collect::<Vec<_>>()
            .join("-|-")
    );

    let mut total_score_for_solution = HashMap::new();
    for (seed, result_for_solution) in sorted_results {
        let min_score = result_for_solution.values().map(|r| r.score).min().unwrap();
        let primary_result = &result_for_solution[primary_solution];
        println!(
            "| {:>4} | {:>2} | {} |",
            seed,
            primary_result.n,
            solutions
                .iter()
                .map(|&solution| {
                    let result = &result_for_solution[solution];
                    format!(
                        "{:>10} / {:>10.8}",
                        result.score,
                        min_score as f64 / result.score as f64
                    )
                    .to_string()
                })
                .collect::<Vec<_>>()
                .join(" | ")
        );

        for &solution in solutions {
            let result = &result_for_solution[solution];
            let entry = total_score_for_solution
                .entry(solution.to_string())
                .or_insert(0.0);
            *entry += min_score as f64 / result.score as f64;
        }
    }

    println!(
        "|------|----|-{}-|",
        solutions
            .iter()
            .map(|_| format!("{:-^width$}", "", width = solution_width))
            .collect::<Vec<_>>()
            .join("-|-")
    );

    println!(
        "|     total | {} |",
        solutions
            .iter()
            .map(|&solution| format!("{:>10.7}", total_score_for_solution[solution]))
            .collect::<Vec<_>>()
            .join(" | ")
    );
}

pub fn main() -> Result<()> {
    let script_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
    let pickle_path = script_dir.join("results.json");

    let mut results: HashMap<String, HashMap<String, TestResult>> = if pickle_path.exists() {
        let file = fs::File::open(&pickle_path)?;
        serde_json::from_reader(file)?
    } else {
        HashMap::new()
    };

    for &solution in SOLUTIONS {
        let env = TestEnvironment::new(solution.to_string())?;
        let seeds = env.seeds.clone();
        if solution != PRIMARY_SOLUTION
            && results.contains_key(solution)
            && results[solution]
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                == seeds
        {
            continue;
        }

        results.insert(solution.to_string(), env.try_solution()?);
    }

    print_results(&results, SOLUTIONS, PRIMARY_SOLUTION);

    let file = fs::File::create(&pickle_path)?;
    serde_json::to_writer_pretty(file, &results)?;

    Ok(())
}
