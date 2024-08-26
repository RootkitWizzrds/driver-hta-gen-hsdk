use std::fs;
use hta_gen::utils::{read_file_to_vec, write_file};

#[test]
fn test_file_operations() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let test_content = b"test content";

    write_file(temp_file.path().to_str().unwrap(), test_content).unwrap();

    let content = read_file_to_vec(temp_file.path().to_str().unwrap()).unwrap();
    assert_eq!(content, test_content);
}
