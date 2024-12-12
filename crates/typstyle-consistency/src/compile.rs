use anyhow::Context;
use reflexo_typst::{CompileDriver, Compiler};
use reflexo_world::{
    config::CompileOpts, CompilerUniverse, EntryOpts, ShadowApi, TypstSystemUniverse,
};
use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::CompilationResult;

pub fn make_universe(content: &str) -> anyhow::Result<TypstSystemUniverse> {
    let root = os_root();
    let mut univ = TypstSystemUniverse::new(CompileOpts {
        entry: EntryOpts::new_rooted(root.clone(), Some(PathBuf::from("/main.typ"))),
        with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
        ..Default::default()
    })?
    .with_entry_file(root.join("main.typ"));
    univ.map_shadow(&root.join("main.typ"), content.as_bytes().into())?;
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
        let content = fs::read(path)?;
        world.map_shadow(&root.join(rel_path), content.clone().into())?;
        formatted_world.map_shadow(
            &root.join(rel_path),
            if path.extension() == Some("typ".as_ref())
                && !blacklist.contains(
                    path.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .as_str(),
                )
            {
                let content = String::from_utf8(content)?;
                formatter(content, rel_path).as_bytes().into()
            } else {
                content.into()
            },
        )?;
    }
    Ok((world, formatted_world))
}

pub fn compile_universe(
    universe: TypstSystemUniverse,
) -> (CompilationResult, CompileDriver<impl Compiler>) {
    let mut driver = CompileDriver::new(PhantomData, universe);
    let doc = driver.compile(&mut Default::default()).map(|x| x.output);
    (doc, driver)
}

fn os_root() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    }
}
