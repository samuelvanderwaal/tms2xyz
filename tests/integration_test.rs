use std::process::Command;
use std::path::Path;

static WITHOUT_ARGS_OUTPUT: &'static str = "error: The following required arguments were not provided:
    <DIR>

USAGE:
    tms2xyz [FLAGS] [OPTIONS] <DIR>

For more information try --help
";

static VERBOSE_OUTPUT: &'static str = 
"Old path: \"./tests/test_dir/16/5838/48441.png\"
New path: \"./tests/test_dir/16/5838/17094.png\"
Old path: \"./tests/test_dir/16/5838/48442.png\"
New path: \"./tests/test_dir/16/5838/17093.png\"
Old path: \"./tests/test_dir/16/5838/48443.png\"
New path: \"./tests/test_dir/16/5838/17092.png\"
Old path: \"./tests/test_dir/16/5834/48441.png\"
New path: \"./tests/test_dir/16/5834/17094.png\"
Old path: \"./tests/test_dir/16/5834/48442.png\"
New path: \"./tests/test_dir/16/5834/17093.png\"
Old path: \"./tests/test_dir/16/5834/48443.png\"
New path: \"./tests/test_dir/16/5834/17092.png\"
6 files processed!
";

static DRYRUN_OUTPUT: &'static str = 
"Old path: \"./tests/test_dir/16/5838/48441.png\"
New path: \"./tests/test_dir/16/5838/17094.png\"
Old path: \"./tests/test_dir/16/5838/48442.png\"
New path: \"./tests/test_dir/16/5838/17093.png\"
Old path: \"./tests/test_dir/16/5838/48443.png\"
New path: \"./tests/test_dir/16/5838/17092.png\"
Old path: \"./tests/test_dir/16/5834/48441.png\"
New path: \"./tests/test_dir/16/5834/17094.png\"
Old path: \"./tests/test_dir/16/5834/48442.png\"
New path: \"./tests/test_dir/16/5834/17093.png\"
Old path: \"./tests/test_dir/16/5834/48443.png\"
New path: \"./tests/test_dir/16/5834/17092.png\"
0 files processed!
";

#[test]
fn no_args() {
    let output = Command::new("./target/debug/tms2xyz")
        .output()
        .expect("Failed to execute process.");

    assert_eq!(String::from_utf8_lossy(&output.stderr), WITHOUT_ARGS_OUTPUT);
}

#[test]
fn test_dir_jpg() {
    let output = Command::new("./target/debug/tms2xyz")
        .args(&["-t", "jpg", "./tests/test_dir"])
        .output()
        .expect("Failed to execute process.");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "0 files processed!\n");
}

#[test]
fn test_dir_png() {
    let output = Command::new("./target/debug/tms2xyz")
        .args(&["./tests/test_dir"])
        .output()
        .expect("Failed to execute process.");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "6 files processed!\n");

    assert!(Path::new("./tests/test_dir/16/5834/17092.png").exists());
    assert!(Path::new("./tests/test_dir/16/5834/17093.png").exists());
    assert!(Path::new("./tests/test_dir/16/5834/17094.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/17092.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/17093.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/17094.png").exists());

    revert_files();
}

#[test]
fn test_verbose() {
   let output = Command::new("./target/debug/tms2xyz")
        .args(&["-v", "./tests/test_dir"])
        .output()
        .expect("Failed to execute process.");

    assert_eq!(String::from_utf8_lossy(&output.stdout), VERBOSE_OUTPUT);

    revert_files();
}

#[test]
fn test_dryrun() {
    let output = Command::new("./target/debug/tms2xyz")
        .args(&["--dryrun", "./tests/test_dir"])
        .output()
        .expect("Failed to execute process.");

    assert_eq!(String::from_utf8_lossy(&output.stdout), DRYRUN_OUTPUT);

    assert!(Path::new("./tests/test_dir/16/5834/48441.png").exists());
    assert!(Path::new("./tests/test_dir/16/5834/48442.png").exists());
    assert!(Path::new("./tests/test_dir/16/5834/48443.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/48441.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/48442.png").exists());
    assert!(Path::new("./tests/test_dir/16/5838/48443.png").exists());
}

fn revert_files() {
    Command::new("./target/debug/tms2xyz")
        .args(&["./tests/test_dir"])
        .output()
        .expect("Failed to execute process.");
}