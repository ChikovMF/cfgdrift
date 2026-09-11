use cfgdrift::{ConfigKey, ConfigKeySegment, ConfigValue, Difference};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn write_json(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    path
}

fn create_key(name: &str) -> ConfigKey {
    let mut k = ConfigKey::default();
    k.push(ConfigKeySegment::Key(name.to_string()));
    k
}

#[test]
fn identical_files_produce_no_differences() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = write_json(dir.path(), "b.json", r#"{"a": 1}"#);

    let diffs = cfgdrift::compare(&left, &right).unwrap();

    assert_eq!(diffs, vec![]);
}

#[test]
fn differing_value_is_listed() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = write_json(dir.path(), "b.json", r#"{"a": 2}"#);

    let diffs = cfgdrift::compare(&left, &right).unwrap();

    assert_eq!(
        diffs,
        vec![Difference::Mismatch {
            key: create_key("a"),
            left_value: ConfigValue::Integer(1),
            right_value: ConfigValue::Integer(2)
        }]
    );
}

#[test]
fn key_only_in_right_is_listed() {
    let dir = tempfile::tempdir().unwrap();
    let left = write_json(dir.path(), "a.json", r#"{"a": 1}"#);
    let right = write_json(dir.path(), "b.json", r#"{"a": 1, "b": 2}"#);

    let diffs = cfgdrift::compare(&left, &right).unwrap();

    assert_eq!(
        diffs,
        vec![Difference::OnlyInRight {
            key: create_key("b"),
            value: ConfigValue::Integer(2)
        }]
    );
}
