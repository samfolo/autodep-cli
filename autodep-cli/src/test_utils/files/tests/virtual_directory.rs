use crate::test_utils::files::VirtualDirectory;

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_create_virtual_directory() {
    let vd = VirtualDirectory::new(None).unwrap();
    assert!(vd.base_dir().path().exists());
}

#[test]
fn test_create_file_with_no_extension() {
    let vd = VirtualDirectory::new(None).unwrap();
    let file = vd
        .create_file("", "testfile", None, Some("content"))
        .unwrap();
    let file_path = PathBuf::from(file);

    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path).unwrap(), "content\n");
}

#[test]
fn test_create_file_with_default_extension() {
    let vd = VirtualDirectory::new(Some("txt")).unwrap();
    let file = vd
        .create_file("", "testfile", None, Some("content"))
        .unwrap();
    let file_path = PathBuf::from(file);

    assert!(file_path.exists());
    assert!(file_path.extension().unwrap() == "txt");
    assert_eq!(fs::read_to_string(file_path).unwrap(), "content\n");
}

#[test]
fn test_create_file_with_specific_extension() {
    let vd = VirtualDirectory::new(None).unwrap();
    let file = vd
        .create_file("", "testfile", Some(OsStr::new("md")), Some("content"))
        .unwrap();
    let file_path = PathBuf::from(file);

    assert!(file_path.exists());
    assert!(file_path.extension().unwrap() == "md");
    assert_eq!(fs::read_to_string(file_path).unwrap(), "content\n");
}

#[test]
fn test_create_subdir() {
    let vd = VirtualDirectory::new(None).unwrap();
    let subdir = vd.create_subdir("./test").unwrap();
    assert!(PathBuf::from(subdir).exists());
}

#[test]
fn test_cwd() {
    let vd = VirtualDirectory::new(None).unwrap();
    assert_eq!(
        vd.cwd().unwrap(),
        vd.base_dir().path().canonicalize().unwrap()
    );
}
