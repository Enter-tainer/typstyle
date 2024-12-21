mod common;

use common::{typstyle_cmd_snapshot, Workspace};

#[test]
fn test_one() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0");
}

#[test]
fn test_one_inplace() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("-i"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
}

#[test]
fn test_one_quiet() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("-q"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0");
}

#[test]
fn test_one_check_quiet() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("--check").arg("-q"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0");
}

#[test]
fn test_two_0() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    #let b = 1


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_1() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    #let b = 1


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b  =  1\n");
}

#[test]
fn test_two_2() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    #let b = 1


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0\n");
    assert_eq!(space.read_string("b.typ"), "#let b  =  1\n");
}

#[test]
fn test_two_0_inplace() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("-i"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    #let b = 1


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_1_inplace() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("-i"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0


    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_2_inplace() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("-i"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_0_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("--check"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_1_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a = 0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("--check"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: b.typ

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b  =  1\n");
}

#[test]
fn test_two_2_check() {
    let space = Workspace::new();
    space.write("a.typ", "#let a  =  0\n");
    space.write("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().arg("a.typ").arg("b.typ").arg("--check"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: a.typ
    Would reformat: b.typ

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a  =  0\n");
    assert_eq!(space.read_string("b.typ"), "#let b  =  1\n");
}
