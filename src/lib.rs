extern crate serialize;
extern crate flate2;
extern crate curl;
extern crate tar;

use std::io::{BufReader, File, SeekSet};
use self::flate2::reader::GzDecoder;
use self::tar::Archive;
use self::curl::http;

pub enum DowntarError {
  HttpError,
  UntarError
}

pub fn download (url: String, dest: String) -> Result<Path, DowntarError> {
  let dest_path = Path::new(dest.as_slice());

  // Attempt HTTP request
  let res = match http::handle().get(url).exec() {
    Ok(res) => res,
    Err(_) => return Err(DowntarError::HttpError)
  };

  // Attempt gzip decoded unpack
  let mut gzipped = GzDecoder::new(BufReader::new(res.get_body()));
  let untar = match Archive::new(gzipped).unpack(&dest_path) {
    Ok(_) => {},
    Err(_) => return Err(DowntarError::UntarError)
  };

  Ok(dest_path)
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
