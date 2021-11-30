use crate::{common, ARGS};

const FILENAME: &str = "rust-toolchain";
const CONTENTS: &str = include_str!("../resources/rust-toolchain.in");

pub fn create() {
    common::write_file(ARGS.root_path().join(FILENAME), CONTENTS);
}

#[cfg(test)]
mod tests {
    use reqwest::blocking;

    use super::CONTENTS;

    const CASPER_NODE_TOOLCHAIN_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/smart_contracts/rust-toolchain";

    #[test]
    fn check_toolchain_version() {
        let expected_toolchain_value = blocking::get(CASPER_NODE_TOOLCHAIN_URL)
            .expect("should get rust-toolchain from GitHub")
            .text()
            .expect("should parse rust-toolchain");

        // If this fails, ensure ../resources/rust-toolchain.in is updated to match the value in
        // "casper-node/rust-toolchain".
        assert_eq!(&*expected_toolchain_value, CONTENTS);
    }
}
