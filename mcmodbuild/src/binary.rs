use crate::structs::{BuildType, ExcludePair, ExcludeType, ModBuild};
use anyhow::{Ok, Result, bail};

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
    let branch = read_cstring(&mut buf)?;

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
        branch,
        build,
        cmd,
        out,
        exclude,
    })
}
