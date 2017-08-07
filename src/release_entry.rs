#[derive(Debug)]
pub struct ReleaseEntry {
  pub is_delta: bool,
  pub sha256: [u8; 32],
  pub length: i64,
}

impl Default for ReleaseEntry {
  fn default() -> ReleaseEntry {
    ReleaseEntry {
      is_delta: true,
      length: 42,
      sha256: [0; 32],
    }
  }
}

impl ReleaseEntry {
  /*
  fn parse(entry: &str) -> Self {
    let mut ret = ReleaseEntry {
      is_delta: false,
      length: 5,
    };
  }

  fn parse(entry: &str) -> Self {
  }

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
  fn stringify_a_sha256() {
    let mut sha = Sha256::default();
    sha.input("This is a test".as_bytes());

    let hash = sha.result();
    print_result(&hash, "SHA256");
    println!("Wat.");
  }
}