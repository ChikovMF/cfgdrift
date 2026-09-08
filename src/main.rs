mod args;
mod config;
mod difference;

use std::path::PathBuf;

fn main() {
    let args = args::Args {
        left_path: PathBuf::from("/home/ChikovMF/Загрузки/test.json"),
        right_path: PathBuf::from("/home/ChikovMF/Загрузки/test2.json"),
    };

    let config_map1 = config::load(&args.left_path);
    let config_map2 = config::load(&args.right_path);

    println!("Config Map 1: {:?}", config_map1);
    println!("Config Map 2: {:?}", config_map2);

    // Здесь будет реализация сравнения конфигураций
    todo!()
}
