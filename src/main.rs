mod cli;
mod structs;

use std::{
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;

use cli::Subcommands;
use oxfmt::Serializable;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    cli::RilipakCli,
    structs::{Pack, PackConfig, PackMeta},
};

fn main() -> Result<()> {
    let cli = RilipakCli::parse();

    match cli.subcommand {
        Subcommands::Build { destination } => build(destination),
        Subcommands::Install { file, destination } => install(file, destination),
        Subcommands::Init { path } => init(path),
    }
}

fn build(destination: Option<PathBuf>) -> Result<()> {
    let content = fs::read_to_string("pack.yml")?;
    let config: PackConfig = serde_yml::from_str(&content)?;
    let id = config.id.clone();
    let destination = destination.unwrap_or(PathBuf::from(format!("{id}.rilipak")));

    let mut modbuilds: Vec<Box<[u8]>> = Vec::new();
    for file in fs::read_dir("include")? {
        let file = file?;
        let content = fs::read(file.path())?;
        modbuilds.push(content.into_boxed_slice());
    }

    let exclude = read_exclude()?;
    let files = zip_dir(Path::new("./"), &exclude)?;

    let pack: Pack = Pack {
        meta: PackMeta { config, modbuilds },
        include: files.into_boxed_slice(),
    };

    let serialized = pack.serialize()?;

    fs::write(destination, serialized)?;

    Ok(())
}

fn read_exclude() -> Result<Vec<PathBuf>> {
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

fn zip_dir(base_dir: &Path, exclude: &[PathBuf]) -> Result<Vec<u8>> {
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

fn install(file: PathBuf, destination: Option<PathBuf>) -> Result<()> {
    todo!()
}

fn init(path: PathBuf) -> Result<()> {
    todo!()
}
