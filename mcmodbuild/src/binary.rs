use crate::cli::McModBuild;
use crate::structs::{BuildType, ExcludePair, ExcludeType, ModBuild};
use anyhow::{Ok, Result, bail};
use clap::Parser;

pub fn serialize(build: ModBuild) -> Result<Vec<u8>> {
    let header_id: &[u8] = "mcmodbuild".as_bytes();
    let header_version: u16 = 1;
    let data_id: &[u8] = build.id.as_bytes();
    let data_name: &[u8] = build.name.as_bytes();
    let data_git: &[u8] = build.git.as_bytes();
    let data_build_type: u8 = build.build as u8;
    let data_cmd: Option<&[u8]> = if build.build == BuildType::Cmd {
        let cmd = build.cmd.as_ref().unwrap();
        Some(cmd.as_bytes())
    } else {
        None
    };
    let data_out: &[u8] = build.out.as_bytes();
    let data_exclude_len: u8 = build.exclude.len() as u8;
    let data_exclude: Vec<u8> = build
        .exclude
        .iter()
        .flat_map(|exclude_pair| {
            let typeint = exclude_pair.type_name.clone() as u8;
            let value = exclude_pair.value.as_bytes();
            let mut buf = Vec::with_capacity(1 + value.len() + 1);
            buf.push(typeint);
            buf.extend(value);
            buf.push(0);
            buf
        })
        .collect();

    let mut buf: Vec<u8> = vec![];
    buf.extend(header_id);
    buf.push(0);
    buf.extend(header_version.to_le_bytes());
    buf.extend(data_id);
    buf.push(0);
    buf.extend(data_name);
    buf.push(0);
    buf.extend(data_git);
    buf.push(0);
    buf.push(data_build_type);
    if let Some(cmd) = data_cmd {
        buf.extend(cmd);
        buf.push(0);
    } else {
        buf.push(0);
    };
    buf.extend(data_out);
    buf.push(0);
    buf.push(data_exclude_len);
    buf.extend(data_exclude);

    Ok(buf)
}

pub fn deserialize(mut buf: &[u8]) -> Result<ModBuild> {
    // helper to read up to the next 0 byte
    fn read_cstring(buf: &mut &[u8]) -> Result<String> {
        if let Some(pos) = buf.iter().position(|&b| b == 0) {
            let s = std::str::from_utf8(&buf[..pos])?.to_string();
            *buf = &buf[pos + 1..];
            Ok(s)
        } else {
            bail!("unterminated c-string");
        }
    }

    // 1) header
    let header = read_cstring(&mut buf)?;
    if header != "mcmodbuild" {
        bail!("invalid header: {}", header);
    }

    // 2) version
    if buf.len() < 2 {
        bail!("buffer too short for version");
    }
    let version = u16::from_le_bytes([buf[0], buf[1]]);
    buf = &buf[2..];
    if version != 1 {
        bail!("unsupported version: {}", version);
    }

    // 3) data fields
    let id = read_cstring(&mut buf)?;
    let name = read_cstring(&mut buf)?;
    let git = read_cstring(&mut buf)?;

    // build type
    if buf.is_empty() {
        bail!("missing build type");
    }
    let build = BuildType::try_from(buf[0])?;
    buf = &buf[1..];

    // optional cmd (always terminated by 0)
    let cmd = {
        let s = read_cstring(&mut buf)?;
        if build == BuildType::Cmd {
            Some(s)
        } else if s.is_empty() {
            None
        } else {
            bail!("unexpected cmd for non-Cmd build");
        }
    };

    // out
    let out = read_cstring(&mut buf)?;

    // exclude count
    if buf.is_empty() {
        bail!("missing exclude count");
    }
    let exclude_len = buf[0] as usize;
    buf = &buf[1..];

    // excludes
    let mut exclude = Vec::with_capacity(exclude_len);
    for _ in 0..exclude_len {
        if buf.is_empty() {
            bail!("truncated exclude entry");
        }
        let t = ExcludeType::try_from(buf[0])?;
        buf = &buf[1..];
        let val = read_cstring(&mut buf)?;
        exclude.push(ExcludePair {
            type_name: t,
            value: val,
        });
    }

    Ok(ModBuild {
        id,
        name,
        git,
        build,
        cmd,
        out,
        exclude,
    })
}
