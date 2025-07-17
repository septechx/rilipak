use std::{
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

use zip::{CompressionMethod, ZipWriter, write::FileOptions};

pub fn read_exclude() -> Result<Vec<PathBuf>> {
    let mut exclude = vec![
        PathBuf::from(".packignore"),
        PathBuf::from("include/"),
        PathBuf::from("pack.yml"),
    ];

    if fs::exists(".packignore")? {
        for line in fs::read_to_string(".packignore")?.split("\n") {
            exclude.push(PathBuf::from(line));
        }
    }

    Ok(exclude)
}

pub fn zip_dir(base_dir: &Path, exclude: &[PathBuf]) -> Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buf);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer_file = Vec::new();

    visit_dirs(
        base_dir,
        base_dir,
        &mut zip,
        &options,
        &mut buffer_file,
        exclude,
    )?;

    zip.finish()?;
    Ok(buf.into_inner())
}

fn visit_dirs(
    base_dir: &Path,
    path: &Path,
    zip: &mut ZipWriter<&mut Cursor<Vec<u8>>>,
    options: &FileOptions<()>,
    buffer: &mut Vec<u8>,
    exclude: &[PathBuf],
) -> Result<()> {
    if exclude.contains(&path.to_path_buf()) {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            visit_dirs(base_dir, &path, zip, options, buffer, exclude)?;
        }
    } else {
        let name = path.strip_prefix(base_dir).unwrap();

        zip.start_file(name.to_string_lossy(), *options)?;
        let mut f = File::open(path)?;
        f.read_to_end(buffer)?;
        zip.write_all(&*buffer)?;
        buffer.clear();
    }

    Ok(())
}
