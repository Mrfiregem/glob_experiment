use std::{io::Write, path::PathBuf, process::exit, sync::Arc};

use glob_experiment::{compiler, globber, matcher, parser};

fn main() {
    const USAGE: &str = "Usage: glob_experiment <pattern> <parse|compile|matches|glob> [path]";

    env_logger::init();

    let mut args = std::env::args_os().skip(1);

    let pattern_string = match args.next() {
        Some(pattern) => pattern,
        None => {
            eprintln!("{}", USAGE);
            exit(1);
        }
    };

    match args.next().map(|s| s.into_encoded_bytes()).as_deref() {
        Some(b"parse") => {
            let pattern = parser::parse(pattern_string);
            println!("{:#?}", pattern);
        }
        Some(b"compile") => {
            let pattern = parser::parse(pattern_string);
            let program = match compiler::compile(&pattern) {
                Ok(program) => program,
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };
            print!("{}", program);
        }
        Some(b"matches") => {
            let path: PathBuf = match args.next() {
                Some(path) => path.into(),
                None => {
                    eprintln!("{}", USAGE);
                    exit(1);
                }
            };
            let pattern = parser::parse(pattern_string);
            let program = match compiler::compile(&pattern) {
                Ok(program) => program,
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };
            let result = matcher::path_matches(&path, &program);
            print!("{:?}", result);
        }
        Some(b"glob") => {
            let pattern = parser::parse(pattern_string);
            let program = match compiler::compile(&pattern) {
                Ok(program) => Arc::new(program),
                Err(err) => {
                    eprintln!("Error compiling pattern: {}", err);
                    exit(1);
                }
            };
            let current_dir = match std::env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Error getting current directory: {}", err);
                    exit(1);
                }
            };
            let mut stdout = std::io::stdout();
            let mut failed = false;
            for result in globber::glob(current_dir, program) {
                match result {
                    Ok(path) => {
                        failed = failed
                            || stdout
                                .write_all(path.as_os_str().as_encoded_bytes())
                                .is_err();
                        failed = failed || stdout.write_all(b"\n").is_err();
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        failed = true;
                    }
                }
            }
            if failed {
                exit(1);
            }
        }
        _ => {
            eprintln!("{}", USAGE);
            exit(1);
        }
    }
}
