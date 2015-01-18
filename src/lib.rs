extern crate serialize;
extern crate flate2;
extern crate curl;
extern crate tar;

use self::flate2::reader::GzDecoder;
use std::io::{BufReader, IoResult};
use self::tar::Archive;
use self::curl::http;

pub enum DowntarError {
  HttpError,
  UntarError
}

fn untar_stream (stream: BufReader, dest: &Path) -> IoResult<()> {
  let mut gzipped = GzDecoder::new(stream);
  let untar = try!(Archive::new(gzipped).unpack(dest));
  Ok(())
}

pub fn download (url: String, dest: Path) -> Result<(), DowntarError> {
  let res = match http::handle().get(url).exec() {
    Ok(body) => body,
    Err(_) => return Err(DowntarError::HttpError)
  };

  match untar_stream(BufReader::new(res.get_body()), &dest) {
    Ok(_) => {},
    Err(_) => return Err(DowntarError::UntarError)
  };

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::io::fs;

  #[test]
  fn get_file () {
    let url = "https://wiki.mozilla.org/images/f/ff/Example.json.gz";
    super::download(url.to_string(), ".".to_string());
  }
}
