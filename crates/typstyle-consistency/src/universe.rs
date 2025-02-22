use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use reflexo_typst::Bytes;
use reflexo_world::{
    config::CompileOpts, CompilerUniverse, EntryOpts, ShadowApi, TypstSystemUniverse,
};

pub fn make_universe(content: &str) -> anyhow::Result<TypstSystemUniverse> {
    let root = os_root();
    let mut univ = TypstSystemUniverse::new(CompileOpts {
        entry: EntryOpts::new_rooted(root.clone(), Some(PathBuf::from("/main.typ"))),
        with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
        ..Default::default()
    })?
    .with_entry_file(root.join("main.typ"));
    univ.map_shadow(
        &root.join("main.typ"),
        Bytes::from_string(content.to_string()),
    )?;
    Ok(univ)
}

pub fn make_universe_formatted(
    source_dir: &Path,
    entrypoint: &Path,
    blacklist: &HashSet<String>,
    formatter: fn(String, &Path) -> String,
) -> anyhow::Result<(TypstSystemUniverse, TypstSystemUniverse)> {
    let root = os_root();
    let entry_file = root.join(
        entrypoint
            .strip_prefix(source_dir)
            .context("entrypoint is not within the testcase_dir")?,
    );

    let make_world = || -> anyhow::Result<TypstSystemUniverse> {
        let univ = CompilerUniverse::new(CompileOpts {
            entry: EntryOpts::new_rooted(root.clone(), Some(entrypoint.to_path_buf())),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..Default::default()
        })?
        .with_entry_file(entry_file.to_path_buf());
        Ok(univ)
    };

    let mut world = make_world()?;
    let mut formatted_world = make_world()?;
    // map all files within the testcase_dir
    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let rel_path = path.strip_prefix(source_dir)?;
        let content = Bytes::new(fs::read(path)?);
        world.map_shadow(&root.join(rel_path), content.clone())?;
        formatted_world.map_shadow(
            &root.join(rel_path),
            if path.extension() == Some("typ".as_ref())
                && !blacklist.contains(path.file_name().unwrap().to_str().unwrap())
            {
                Bytes::new(formatter(content.as_str().unwrap().to_string(), rel_path))
            } else {
                content
            },
        )?;
    }
    Ok((world, formatted_world))
}

fn os_root() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    }
}
