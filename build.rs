use std::{ffi::OsStr, fs, io, path::PathBuf};

fn find_sources(basedir: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for res in fs::read_dir(basedir)? {
        let ent = res?;
        let path = ent.path();
        if ent.file_type()?.is_file() && path.extension() == Some(OsStr::new("c")) {
            files.push(path);
        }
    }
    Ok(files)
}

fn main() -> io::Result<()> {
    let mylib_src = find_sources("TestU01-1.2.3/mylib")?;
    let probdist_src = find_sources("TestU01-1.2.3/probdist")?;
    let testu01_src = find_sources("TestU01-1.2.3/testu01")?;

    cc::Build::new()
        .include("TestU01-1.2.3/include")
        .files(mylib_src)
        .files(probdist_src)
        .files(testu01_src)
        .compile("testu01");
    Ok(())
}
