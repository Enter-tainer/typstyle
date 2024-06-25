use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    // Emit the instructions
    let mut builder = EmitBuilder::builder();

    builder.all_cargo().build_timestamp().all_rustc();

    #[cfg(feature = "git-info")]
    builder.git_sha(false).git_describe(true, true, None);

    builder.emit()?;
    Ok(())
}
