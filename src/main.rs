mod args;
mod exit_status;
mod report;

use crate::args::Args;
use crate::exit_status::ExitStatus;
use cfgdrift::compare;
use clap::Parser;

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
            let _ = report::print_diff(&mut anstream::stdout(), &diffs);
            if diffs.is_empty() {
                ExitStatus::Identical
            } else {
                ExitStatus::Drift
            }
        }
        Err(err) => {
            let _ = report::print_error(&mut anstream::stderr(), &err);
            ExitStatus::Error
        }
    }
}
