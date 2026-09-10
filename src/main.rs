mod args;
mod exit_status;
mod report;

use crate::args::Args;
use cfgdrift::compare;
use clap::Parser;
use crate::exit_status::ExitStatus;

fn main() -> ExitStatus {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            let _ = err.print();
            return if err.use_stderr() {
                ExitStatus::Error
            } else {
                ExitStatus::Identical
            };
        }
    };

    match compare(&args.left_path, &args.right_path) {
        Ok(diffs) => {
            let _ = report::print_diff(&mut std::io::stdout(), &diffs);
            if diffs.is_empty() {
                ExitStatus::Identical
            } else {
                ExitStatus::Drift
            }
        }
        Err(err) => {
            let _ = report::print_error(&mut std::io::stderr(), &err);
            ExitStatus::Error
        }
    }
}
