mod args;
mod args_error;

use cfgdrift::compare;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = match args::Args::parse() {
        Ok(args) => args,
        Err(err) => {
            report(&err);
            return ExitCode::from(2);
        }
    };

    match compare(&args.left_path, &args.right_path) {
        Ok(diffs) if diffs.is_empty() => {
            println!("конфигурации идентичны");
            ExitCode::SUCCESS
        }
        Ok(diffs) => {
            println!("конфигурации различаются:");
            for diff in diffs {
                println!("{:?}", diff);
            }
            ExitCode::from(1)
        }
        Err(err) => {
            report(&err);
            ExitCode::from(2)
        }
    }
}

fn report(err: &dyn std::error::Error) {
    eprintln!("ошибка: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        eprintln!("  причина: {cause}");
        source = cause.source();
    }
}
