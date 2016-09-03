extern crate funiq;

use funiq::process_files;

#[test]
fn finds_unique_files() {
    let (unique, duplicate) = process_files(&["./tests/fixtures/original_a.txt",
                                 "./tests/fixtures/original_b.txt",
                                 "./tests/fixtures/duplicate_a.txt"]).unwrap();

    assert_eq!(unique.len(), 2);
    assert_eq!(duplicate.len(), 1);

    let (unique, duplicate) = process_files(&["./tests/fixtures/original_a.txt",
                                 "./tests/fixtures/original_b.txt",
                                 "./tests/fixtures/original_c.txt"]).unwrap();

    assert_eq!(unique.len(), 3);
    assert_eq!(duplicate.len(), 0);
}

#[test]
fn should_fail() {
    let result = process_files(&["./tests/fixtures/original_a.txt",
                                 "./tests/fixtures/original_b.txt",
                                 "./tests/fixtures/duplicate_a.txt",
                                 "./tests/fixtures/does_not_exist.txt"]);

    assert!(result.is_err());
}
