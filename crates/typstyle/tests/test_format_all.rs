mod common;

use common::{typstyle_cmd_snapshot, Workspace};

#[test]
fn test_all_0() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Successfully formatted 2 files (0 unchanged) in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b = 1\n");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_1() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Successfully formatted 1 files (1 unchanged) in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b = 1\n");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_2() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b = 1\n");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Successfully formatted 0 files (2 unchanged) in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b = 1\n");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_0_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("--check").arg("-v"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: [TEMP_PATH]/project/a.typ
    Would reformat: [TEMP_PATH]/project/x/b.typ
    2 files would be reformatted (0 already formatted), checked in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0");
    assert_eq!(space.read_string("x/b.typ"), "#let b  =  1");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_1_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("--check").arg("-v"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: [TEMP_PATH]/project/x/b.typ
    1 files would be reformatted (1 already formatted), checked in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b  =  1");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_2_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b = 1\n");
    space.write("x/y/.c.typ", "#let c  =  2");
    space.write("x/.z/d.typ", "#let d  =  3");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("--check").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0 files would be reformatted (2 already formatted), checked in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b = 1\n");
    assert_eq!(space.read_string("x/y/.c.typ"), "#let c  =  2");
    assert_eq!(space.read_string("x/.z/d.typ"), "#let d  =  3");
}

#[test]
fn test_all_erroneous() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/c.typ", "#let c  =  2; #");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Successfully formatted 1 files (1 unchanged) in [DURATION]

    ----- stderr -----
    warn: Failed to format: [TEMP_PATH]/project/x/y/c.typ
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b = 1\n");
    assert_eq!(space.read_string("x/y/c.typ"), "#let c  =  2; #");
}

#[test]
fn test_all_erroneous_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/c.typ", "#let c  =  2; #");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("--check").arg("-v"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: [TEMP_PATH]/project/x/b.typ
    1 files would be reformatted (1 already formatted), checked in [DURATION]

    ----- stderr -----
    warn: Failed to format: [TEMP_PATH]/project/x/y/c.typ
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("x/b.typ"), "#let b  =  1");
    assert_eq!(space.read_string("x/y/c.typ"), "#let c  =  2; #");
}

#[test]
fn test_all_column() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  (1 + 2)");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("-c=0").arg("-v"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Successfully formatted 1 files (0 unchanged) in [DURATION]

    ----- stderr -----
    ");

    assert_eq!(
        space.read_string("a.typ"),
        "#let a = (
  1
    + 2
)
"
    );
}

#[test]
fn test_dir_all_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");
    space.write("x/b.typ", "#let b  =  1");
    space.write("x/y/.c.typ", "#let c  =  2");

    typstyle_cmd_snapshot!(space.cli().arg("format-all").arg("x").arg("--check").arg("-v"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: x/b.typ
    1 files would be reformatted (0 already formatted), checked in [DURATION]

    ----- stderr -----
    ");
}
