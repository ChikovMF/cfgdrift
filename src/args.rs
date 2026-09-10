use clap::Parser;
use std::path::PathBuf;

/// Аргументы.
#[derive(Parser)]
#[command(
    version,
    about = "Сравнивает два конфига и показывает дрейф между ними"
)]
pub struct Args {
    /// Путь к первой конфигурации.
    pub left_path: PathBuf,

    /// Путь ко второй конфигурации.
    pub right_path: PathBuf,
}
