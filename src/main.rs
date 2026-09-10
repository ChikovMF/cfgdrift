mod args;
mod args_error;
mod report;

use cfgdrift::compare;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = match args::Args::parse() {
        Ok(args) => args,
        Err(err) => {
            let _ = report::print_error(&mut std::io::stderr(), &err);
            return ExitCode::from(2);
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
