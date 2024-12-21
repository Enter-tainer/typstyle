use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use insta_cmd::get_cargo_bin;
use tempfile::TempDir;

pub const INSTA_FILTERS: &[(&str, &str)] = &[
    (r"(?m)^(.+) in [\w.]+$", "$1 in [DURATION]"),
    (
        r"(\b[A-Z]:)?[\\/].*?[\\/]\.typstyle-tests---[^\\/]+[\\/]",
        "[TEMP_PATH]/",
    ),
    (r"\\\\?([\w\d.])", "/$1"),
    (r"typstyle.exe", "typstyle"),
];

pub struct Workspace {
    #[allow(unused)]
    tempdir: TempDir,
    project_dir: PathBuf,
}

#[allow(dead_code)]
impl Workspace {
    pub fn new() -> Workspace {
        let tempdir = marked_tempdir();
        let project_dir = tempdir.path().join("project");
        fs::create_dir_all(&project_dir).unwrap();
        Workspace {
            tempdir,
            project_dir,
        }
    }

    pub fn project_path(&self) -> &Path {
        &self.project_dir
    }

    pub fn cmd(&self, cmd: impl AsRef<OsStr>) -> Command {
        let mut rv = Command::new(cmd);
        rv.current_dir(self.project_path());
        rv
    }

    pub fn cli(&self) -> Command {
        self.cmd(get_cargo_bin("typstyle"))
    }

    pub fn write(&self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) {
        let p = self.project_path().join(path.as_ref());
        fs::create_dir_all(p.parent().unwrap()).ok();
        fs::write(p, contents).unwrap();
    }

    pub fn read_string(&self, path: impl AsRef<Path>) -> String {
        let p = self.project_path().join(path.as_ref());
        fs::read_to_string(p).unwrap()
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

fn marked_tempdir() -> TempDir {
    TempDir::with_prefix(".typstyle-tests---").unwrap()
}

#[allow(unused)]
macro_rules! typstyle_cmd_snapshot {
    ($cmd:expr, @$snapshot:literal) => {
        let mut settings = insta::Settings::clone_current();
        for (matcher, replacement) in $crate::common::INSTA_FILTERS {
            settings.add_filter(matcher, *replacement);
        }
        let _guard = settings.bind_to_scope();
        insta_cmd::assert_cmd_snapshot!($cmd, @$snapshot);
    };
}

#[allow(unused_imports)]
pub(crate) use typstyle_cmd_snapshot;
