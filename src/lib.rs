extern crate serialize;
extern crate flate2;
extern crate curl;
extern crate tar;

use std::io::{fs, BufReader, File, SeekSet, IoResult};
use self::flate2::reader::GzDecoder;
use self::tar::Archive;
use self::curl::http;

pub enum DowntarError {
  HttpError,
  UntarError
}

fn untar_stream (stream: BufReader, dest: &Path) -> IoResult<()> {
  let mut gzipped = GzDecoder::new(stream);

  let tmp_folder = Path::new("./tmp");

  let untar = try!(Archive::new(gzipped).unpack(&tmp_folder));
  let temp_contents = try!(fs::readdir(&tmp_folder));

  let first = temp_contents.iter().next().unwrap();
  println!("first: {:?}, dest: {:?}", first, dest);
  // fs::rename(first, dest);

  Ok(())
}

pub fn download (url: String, dest: String) -> Result<Path, DowntarError> {
  let dest_path = Path::new(dest.as_slice());

  let res = match http::handle().get(url).exec() {
    Ok(body) => body,
    Err(_) => return Err(DowntarError::HttpError)
  };

  match untar_stream(BufReader::new(res.get_body()), &dest_path) {
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
