use hex::*;
use regex::Regex;
use semver::Version;
use std::iter::*;
use std::error::{Error};
use url::{Url};
use url::percent_encoding::{percent_decode};

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
  pub version: Version,
  pub length: i64,
  pub is_delta: bool,
  pub percentage: i32,
}

impl Default for ReleaseEntry {
  fn default() -> ReleaseEntry {
    ReleaseEntry {
      filename_or_url: "Foobdar".to_owned(),
      version: Version::parse("1.0.0").unwrap(),
      is_delta: true,
      length: 42,
      sha256: [0; 32],
      percentage: 100,
    }
  }
}

lazy_static! {
  static ref SCHEME: Regex = Regex::new(r"^https:").unwrap();
}

lazy_static! {
  static ref COMMENT: Regex = Regex::new(r"#.*$").unwrap();
}

impl ReleaseEntry {
  fn parse_sha256(sha256: &str, to_fill: &mut ReleaseEntry) -> Result<bool, Box<Error>> {
    let ret = try!(Vec::from_hex(sha256));
    if ret.len() != 32 {
      return Err(From::from("SHA256 is malformed"));
    }

    for i in 0..32 { to_fill.sha256[i] = ret[i]; }
    return Ok(true);
  }

  fn parse_delta_full(delta_or_full: &str) -> Result<bool, Box<Error>> {
    match delta_or_full {
      "delta" => Ok(true),
      "full" => Ok(false),
      _ => Err(From::from("Package type must be either 'delta' or 'full'"))
    }
  }

  fn parse_name(filename_or_url: &str) -> Result<String, Box<Error>> {
    if SCHEME.is_match(filename_or_url) {
      try!(Url::parse(filename_or_url));
      return Ok(filename_or_url.to_owned())
    } else {
      let u = format!("file:///{}", filename_or_url);
      let url = try!(Url::parse(&u));

      let decoded = try!(percent_decode(url.path().as_bytes()).decode_utf8());
      return Ok(decoded.trim_left_matches("/").to_owned());
    }
  }

  fn parse_percentage(percent: &str) -> Result<i32, Box<Error>> {
    let n = try!(percent.trim_right_matches("%").parse::<i32>());
    if n > 100 || n < 0 {
      return Err(From::from("Percentage must be between 0 and 100 inclusive"));
    }

    return Ok(n);
  }

  pub fn parse(entry: &str) -> Result<Self, Box<Error>> {
    let e = entry.split_whitespace().collect::<Vec<_>>();

    return match e.len() {
      5 => {
        let (sha256, name, version, size, delta_or_full) = (e[0], e[1], e[2], e[3], e[4]);
        let mut ret = ReleaseEntry {
          sha256: [0; 32],
          is_delta: try!(ReleaseEntry::parse_delta_full(delta_or_full)),
          filename_or_url: try!(ReleaseEntry::parse_name(name)),
          version: try!(Version::parse(version)),
          length: try!(size.parse::<i64>()),
          percentage: 100,
        };

        try!(ReleaseEntry::parse_sha256(sha256, &mut ret));
        return Ok(ret);
      },
      6 => {
        let (sha256, name, version, size, delta_or_full, percent) = (e[0], e[1], e[2], e[3], e[4], e[5]);
        let mut ret = ReleaseEntry {
          sha256: [0; 32],
          is_delta: try!(ReleaseEntry::parse_delta_full(delta_or_full)),
          filename_or_url: try!(ReleaseEntry::parse_name(name)).to_owned(),
          version: try!(Version::parse(version)),
          length: try!(size.parse::<i64>()),
          percentage: try!(ReleaseEntry::parse_percentage(percent))
        };

        try!(ReleaseEntry::parse_sha256(sha256, &mut ret));
        return Ok(ret);
      },
      _ => Err(From::from("Invalid Release Entry string"))
    }
  }

  pub fn parse_entries(content: &str) -> Result<Vec<ReleaseEntry>, Box<Error>> {
    let mut was_error: Option<Box<Error>> = None;

    let r: Vec<ReleaseEntry> = content.split("\n").filter_map(|x| {
      let r = COMMENT.replace_all(x, "");
      if r.len() == 0 {
        return None;
      }

      match ReleaseEntry::parse(&r) {
        Err(err) => {
          was_error = Some(err);
          return None;
        },
        Ok(val) => Some(val)
      }
    }).collect();

    return match was_error {
      Some(err) => Err(err),
      None => Ok(r)
    };
  }
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
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 full";
    let result = ReleaseEntry::parse(input).unwrap();

    assert_eq!(result.sha256[0], 0xE4);
    assert_eq!(result.sha256[1], 0x54);
    assert_eq!(result.sha256[31], 0x35);
  }

  #[test]
  fn parse_should_fail_invalid_sha256() {
    let input = "48fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 full";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn parse_should_fail_very_invalid_sha256() {
    let input = "48Z myproject.7z 12345 full";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn parse_should_fail_invalid_type() {
    let input = "48fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 foobar";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn parse_should_set_delta_package() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 delta";
    let result = ReleaseEntry::parse(input).unwrap();

    assert_eq!(result.is_delta, true);

    let input2 = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 full";
    let result2 = ReleaseEntry::parse(input2).unwrap();

    assert_eq!(result2.is_delta, false);
  }

  #[test]
  fn parse_should_accept_percentages() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 delta 45%";
    let result = ReleaseEntry::parse(input).unwrap();
    assert_eq!(result.percentage, 45);
  }

  #[test]
  fn parse_should_fail_giant_percentages() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 delta 145%";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn parse_should_fail_negative_percentages() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 delta -145%";
    ReleaseEntry::parse(input).unwrap_err();
  }

  #[test]
  fn url_encoded_filenames_should_end_up_decoded() {
    let input = "e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 my%20project.7z 1.2.3 12345 full";
    let result = ReleaseEntry::parse(input).unwrap();

    assert_eq!(result.filename_or_url, "my project.7z");
  }

  #[test]
  fn parse_all_entries() {
    let input = "
# SHA256 of the file                                             Name       Version Size  [delta/full] release%
e4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject.7z 1.2.3 12345 full
a4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject-delta.7z 1.2.3 555 delta
b4548fba3f902e63e3fff36db7cbbd1837493e21c51f0751e51ee1483ddd0f35 myproject-beta.7z 2.0.0-beta.1 34567 full 5%";

    let result = ReleaseEntry::parse_entries(input).unwrap();
    assert_eq!(result.len(), 3);
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
