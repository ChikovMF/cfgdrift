use std::process::ExitCode;

pub enum ExitStatus {
    Identical = 0,
    Drift = 1,
    Error = 2,
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