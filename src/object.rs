use std::{
    ffi::CStr,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::Path,
};

use flate2::read::ZlibDecoder;

type ObjectReader = BufReader<ZlibDecoder<File>>;

pub fn create_zlib_reader<P: AsRef<Path>>(path: P) -> anyhow::Result<ObjectReader> {
    let file = fs::File::open(path)?;
    let z = ZlibDecoder::new(file);
    Ok(BufReader::new(z))
}
pub fn read_header(r: &mut ObjectReader) -> anyhow::Result<(String, String)> {
    let mut buf = Vec::new();
    r.read_until(0, &mut buf)?;
    let header = CStr::from_bytes_until_nul(&buf)?.to_str()?;
    let Some((kind, size)) = header.split_once(' ') else {
        anyhow::bail!("Incorrect header");
    };
    Ok((kind.to_string(), size.to_string()))
}
pub fn read_hash(r: &mut ObjectReader) -> anyhow::Result<String> {
    let mut buf = [0; 20];
    r.read_exact(&mut buf)?;
    let hash = hex::encode(buf);
    Ok(hash)
}
pub fn read_until(r: &mut ObjectReader, byte: u8) -> anyhow::Result<(usize, String)> {
    let mut buf = Vec::new();
    let n = r.read_until(byte, &mut buf)?;
    if n == 0 {
        return Ok((n, "".to_string()));
    }
    let s = CStr::from_bytes_until_nul(&buf)?.to_str()?;
    let s = s.to_string();

    Ok((n, s))
}

pub fn read_to_string(r: &mut ObjectReader) -> anyhow::Result<String> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    Ok(s)
}
