use std::path::PathBuf;

use cargonode::ui::Status;

#[test]
fn test_status_new_package() {
    let status = Status::new(true, false, true);
    status.start(&PathBuf::from("test-pkg"));
    status.created_manifest();
    status.created_source_files();
    status.initialized_git();
    status.created_package();
    status.finish("test-pkg");
}

#[test]
fn test_status_init_package() {
    let status = Status::new(false, true, false);
    status.start(&PathBuf::from("test-pkg"));
    status.created_manifest();
    status.created_source_files();
    status.initialized_git();
    status.created_package();
    status.finish("test-pkg");
}

#[test]
fn test_status_workspace() {
    let status = Status::new(false, false, false);
    status.start(&PathBuf::from("test-ws"));
    status.created_workspace();
    status.created_manifest();
    status.initialized_git();
    status.created_package();
    status.finish("test-ws");
}

#[test]
fn test_status_warning() {
    let status = Status::new(false, false, false);
    status.warning("test warning");
}
