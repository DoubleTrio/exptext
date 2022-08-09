use assert_cmd::Command;
use std::fs;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "exptext";
const DB: &str = "--db";
const DB_NAME: &str = "tests/test_actors.json";

const M00: &str = "tests/inputs/m00p0801.txt";
const M01: &str = "tests/inputs/m01a0208.txt";
const M04: &str = "tests/inputs/m04a0101.txt";
const M12: &str = "tests/inputs/m12a0102.txt";
const ALL: [&str; 4] = [M00, M01, M04, M12];

fn init_commands() -> [CommandOptions<'static>; 10] {
    [
        CommandOptions::new(&[], "", "No command options"),
        CommandOptions::new(&["-f"], ".f", "Should not format text"),
        CommandOptions::new(&["-n"], ".n", "Should number lines"),
        CommandOptions::new(&["-s"], ".s", "Should sort lines"),
        CommandOptions::new(&["-fn"], ".fn", "No format and number lines"),
        CommandOptions::new(&["-fs"], ".fs", "No format and sort"),
        CommandOptions::new(&["-ns"], ".ns", "Line number and sort"),
        CommandOptions::new(&["-l", "e"], ".l-e", "Filter by english"),
        CommandOptions::new(&["-nl", "e"], ".nl-e", "Filter by english and number lines"),
        CommandOptions::new(&["-fnsl", "e"], ".fnsl-e", "Combine all options"),
    ]
}

struct CommandOptions<'a> {
    pub options: &'a [&'a str],
    pub file_end: &'a str,
    #[allow(dead_code)]
    pub description: &'a str,
}

impl<'a> CommandOptions<'a> {
    pub fn new(options: &'a [&str], file_end: &'a str, description: &'a str) -> Self {
        Self {
            options,
            file_end,
            description,
        }
    }
}

fn run(args: &[&str], input_files: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args([DB, DB_NAME].iter().chain(input_files).chain(args))
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn handles_single_file() -> TestResult {
    let commands = init_commands();
    for command_op in commands {
        let CommandOptions {
            options,
            file_end,
            description: _,
        } = command_op;
        for file in ALL {
            let path = Path::new(file);
            match run(
                options,
                &[file],
                format!(
                    "tests/expected/{}{}.out",
                    path.file_name().unwrap().to_str().unwrap(),
                    file_end
                )
                .as_str(),
            ) {
                Ok(_) => continue,
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    Ok(())
}

#[test]
fn handles_multiple_files() -> TestResult {
    let commands = init_commands();
    for command_op in commands {
        let CommandOptions {
            options,
            file_end,
            description: _,
        } = command_op;
        match run(
            options,
            &ALL,
            format!("tests/expected/all{}.out", file_end).as_str(),
        ) {
            Ok(_) => continue,
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
