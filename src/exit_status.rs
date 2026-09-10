use std::process::ExitCode;

pub enum ExitStatus {
    Identical,
    Drift,
    Error,
}

impl std::process::Termination for ExitStatus {
    fn report(self) -> ExitCode {
        match self {
            ExitStatus::Identical => ExitCode::from(0),
            ExitStatus::Drift => ExitCode::from(1),
            ExitStatus::Error => ExitCode::from(2),
        }
    }
}
