use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn write_json(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    path
}

#[test]
fn identical_files_exit_with_zero() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = write_json(dir.path(), "b.json", r#"{"a": 1}"#);

    let output = Command::new(env!("CARGO_BIN_EXE_cfgdrift"))
        .arg(&left)
        .arg(&right)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn differing_values_exit_with_one() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = write_json(dir.path(), "b.json", r#"{"a": 2}"#);

    let output = Command::new(env!("CARGO_BIN_EXE_cfgdrift"))
        .arg(&left)
        .arg(&right)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn missing_right_file_exits_with_two() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = PathBuf::new().join(dir.path()).join("b.json");

    let output = Command::new(env!("CARGO_BIN_EXE_cfgdrift"))
        .arg(&left)
        .arg(&right)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
}
