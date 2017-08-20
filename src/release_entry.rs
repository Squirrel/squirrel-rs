use std::error::{Error};
use regex::*;
use hex::*;

/* Example lines:

# SHA256 of the file                                             Name       Version Size  [delta/full] release%
e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 full
a4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject-delta.7z 123 delta
b4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject-beta.7z 34567 full 5%
*/

#[derive(Debug)]
pub struct ReleaseEntry {
  pub sha256: [u8; 32],
  pub filename_or_url: String,
  pub length: i64,
  pub is_delta: bool,
  pub percentage: i64,
}

impl Default for ReleaseEntry {
  fn default() -> ReleaseEntry {
    ReleaseEntry {
      filename_or_url: "Foobar".to_owned(),
      is_delta: true,
      length: 42,
      sha256: [0; 32],
      percentage: 100,
    }
  }
}

lazy_static! {
  static ref SHA256: Regex = Regex::new(r"^[A-Fa-f0-9]{64}$").unwrap();
}

impl ReleaseEntry {
  fn parse_sha256(sha256: &str, to_fill: &mut ReleaseEntry) -> Result<bool, Box<Error>> {
    if SHA256.is_match(sha256) == false {
      return Err(From::from("SHA256 is malformed"));
    }

    let ret = try!(sha256.from_hex());
    if ret.len() != 32 {
      return Err(From::from("SHA256 is malformed"));
    }

    for i in 0..32 { to_fill.sha256[i] = ret[i]; }
    return Ok(true);
  }

  pub fn parse(entry: &str) -> Result<Self, Box<Error>> {
    let mut ret = ReleaseEntry::default();
    let entries = entry.split_whitespace().collect::<Vec<&str>>();

    try!(ReleaseEntry::parse_sha256(entries[0], &mut ret));
    return Ok(ret);
  }

/*
  fn parse_file(file: &str) -> Vec<ReleaseEntry> {
  }
 */
}

#[cfg(test)]
mod tests {
  use sha2::Sha256;
  use sha2::Digest;
  use super::ReleaseEntry;

  fn print_result(sum: &[u8], name: &str) {
    for byte in sum {
      print!("{:02x}", byte);
    }
    println!("\t{}", name);
  }

  #[test]
  fn create_a_release_entry() {
    let f = ReleaseEntry::default();
    assert_eq!(f.length, 42);
  }

  #[test]
  fn parse_should_read_valid_sha256() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 12345 full";
    let result = ReleaseEntry::parse(input).unwrap();

    assert_eq!(result.sha256[0], 0xE4);
    assert_eq!(result.sha256[1], 0x54);
    assert_eq!(result.sha256[31], 0x35);
  }

  #[test]
  fn parse_should_fail_invalid_sha256() {
    let input = "48fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 12345 full";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn parse_should_fail_very_invalid_sha256() {
    let input = "48Z myproject.7z 12345 full";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn stringify_a_sha256() {
    let mut sha = Sha256::default();
    sha.input("This is a test".as_bytes());

    let hash = sha.result();
    print_result(&hash, "SHA256");
    println!("Wat.");
  }
}
