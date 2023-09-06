use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::error::Error;


type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "fixtojson";
const IN_MINLEN: &str = "tests/inputs/minimal_length.txt";
const IN_PREFIXED: &str = "tests/inputs/prefixed.txt";
const IN_CTRLA: &str = "tests/inputs/control_a_seperator.txt";
const IN_NESTED1: &str = "tests/inputs/nested_groups_1.txt";
const IN_GZIPPED: &str = "tests/inputs/gzipped.txt.gz";
const IN_BAD_GZIPPED: &str = "tests/inputs/bad_gzip.txt.gz";

const OUT_MINLEN: &str = "tests/expected/minimal_length.txt";
const OUT_PREFIXED: &str = "tests/expected/prefixed.txt";
const OUT_CTRLA: &str = "tests/expected/control_a_seperator.txt";
const OUT_NESTED1: &str = "tests/expected/nested_groups_1.txt";
const OUT_GZIPPED: &str = "tests/expected/gzipped.txt";

#[test]
fn usage() -> TestResult {
	for flag in &["-h", "--help"] {
		Command::cargo_bin(PRG)?
			.arg(flag)
			.assert()
			.stdout(predicate::str::contains("Usage"));
	}
	Ok(())
}

fn gen_bad_file() -> String {
	loop {
		let filename: String = rand::thread_rng()
			.sample_iter(&Alphanumeric)
			.take(7)
			.map(char::from)
			.collect();

		if fs::metadata(&filename).is_err() {
			return filename;
		}
	}
}

#[test]
fn skips_bad_file() -> TestResult {
	let bad = gen_bad_file();
	let expected = format!("{}: .* [(]os error 2[)]", bad);
	Command::cargo_bin(PRG)?
		.arg(&bad)
		.assert()
		.success()
		.stderr(predicate::str::is_match(expected)?);
	Ok(())
}

#[test]
fn skips_bad_gzip() -> TestResult {
	let expected = "invalid gzip header\n";
	// we expect this to fail when we try to read the non-gzipped file as if it were gzipped
	Command::cargo_bin(PRG)?
		.arg(&IN_BAD_GZIPPED)
		.assert()
		.failure()
		.stderr(predicate::str::is_match(expected)?);
	Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
	let expected = fs::read_to_string(expected_file)?;
	Command::cargo_bin(PRG)?
		.args(args)
		.assert()
		.success()
		.stdout(expected);
	Ok(())
}

fn run_stdin(
	input_file: &str,
	args: &[&str],
	expected_file: &str,
) -> TestResult {
	let input = fs::read_to_string(input_file)?;
	let expected = fs::read_to_string(expected_file)?;
	Command::cargo_bin(PRG)?
		.args(args)
		.write_stdin(input)
		.assert()
		.success()
		.stdout(expected);
	Ok(())
}

#[test]
fn minlen_stdin() -> TestResult {
	run_stdin(IN_MINLEN, &["-"], OUT_MINLEN)
}

#[test]
fn minlen() -> TestResult {
	run(&[IN_MINLEN], OUT_MINLEN)
}

#[test]
fn ctrla() -> TestResult {
	run(&[IN_CTRLA], OUT_CTRLA)
}

#[test]
fn prefixed() -> TestResult {
	run(&[IN_PREFIXED], OUT_PREFIXED)
}

#[test]
fn nested1() -> TestResult {
	run(&[IN_NESTED1], OUT_NESTED1)
}

#[test]
fn gzipped() -> TestResult {
	run(&[IN_GZIPPED], OUT_GZIPPED)
}