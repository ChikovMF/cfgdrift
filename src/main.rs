mod args;
mod report;

use crate::args::Args;
use cfgdrift::compare;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            let _ = err.print();
            return if err.use_stderr() {
                ExitCode::from(2)
            } else {
                ExitCode::SUCCESS
            };
        }
    };

    match compare(&args.left_path, &args.right_path) {
        Ok(diffs) => {
            let _ = report::print_diff(&mut std::io::stdout(), &diffs);
            if diffs.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(err) => {
            let _ = report::print_error(&mut std::io::stderr(), &err);
            ExitCode::from(2)
        }
    }
}
