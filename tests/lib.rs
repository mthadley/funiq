extern crate funiq;

use funiq::process_files;

#[test]
fn finds_unique_files() {
    let files = vec!["./tests/fixtures/original_a.txt".into(),
                     "./tests/fixtures/original_b.txt".into(),
                     "./tests/fixtures/duplicate_a.txt".into()];
    let (unique, duplicate) = process_files(&files).unwrap();

    assert_eq!(unique.len(), 2);
    assert_eq!(duplicate.len(), 1);

    let files = vec!["./tests/fixtures/original_a.txt".into(),
                     "./tests/fixtures/original_b.txt".into(),
                     "./tests/fixtures/original_c.txt".into()];
    let (unique, duplicate) = process_files(&files).unwrap();

    assert_eq!(unique.len(), 3);
    assert_eq!(duplicate.len(), 0);
}

#[test]
fn should_fail() {
    let files = vec!["./tests/fixtures/original_a.txt".into(),
                     "./tests/fixtures/original_b.txt".into(),
                     "./tests/fixtures/duplicate_a.txt".into(),
                     "./tests/fixtures/does_not_exist.txt".into()];
    let result = process_files(&files);

    assert!(result.is_err());
}
