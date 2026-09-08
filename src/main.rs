mod args;
mod config;
mod difference;

use cfgdrift::compare;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = args::Args::parse().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        std::process::exit(1);
    });

    match compare(&args.left_path, &args.right_path) {
        Ok(diffs) if diffs.is_empty() => {
            println!("Configs are identical.");
            ExitCode::SUCCESS
        }
        Ok(diffs) => {
            println!("Configs differ:");
            for diff in diffs {
                println!("{:?}", diff);
            }
            ExitCode::from(1)
        }
        Err(err) => {
            eprintln!("Error comparing configs: {:?}", err);
            ExitCode::from(2)
        }
    }
}
