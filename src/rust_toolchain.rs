use crate::{common, ARGS};

const FILENAME: &str = "rust-toolchain";
const CONTENTS: &str = include_str!("../resources/rust-toolchain.in");

pub fn create() {
    common::write_file(ARGS.root_path().join(FILENAME), CONTENTS);
}

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::blocking;

    use super::CONTENTS;

    const TEST_BRANCH_NAME: &str = "dev";
    const CRON_JOB_BRANCH_NAME_ENV_VAR: &str = "BRANCH_SELECTOR";
    const PR_TARGET_BRANCH_NAME_ENV_VAR: &str = "GITHUB_BASE_REF";
    const CI_BRANCH_NAME_ENV_VAR: &str = "GITHUB_REF_NAME";
    const CASPER_NODE_TOOLCHAIN_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/smart_contracts/rust-toolchain";

    /// Checks that the pinned version of Rust is that of the `dev` branch of casper-node.
    ///
    /// If none of `BRANCH_SELECTOR`, `GITHUB_BASE_REF` and `GITHUB_REF_NAME` are set, or they're
    /// all set to empty strings, the test is an auto-pass.
    ///
    /// For testing locally, you can manually run e.g.
    /// ```
    /// BRANCH_SELECTOR=dev cargo t
    /// ```
    #[test]
    fn check_toolchain_version_on_dev() {
        if let Ok(true) = env::var(CRON_JOB_BRANCH_NAME_ENV_VAR)
            .or_else(|_| env::var(PR_TARGET_BRANCH_NAME_ENV_VAR))
            .or_else(|_| env::var(CI_BRANCH_NAME_ENV_VAR))
            .map(|env_var| env_var == TEST_BRANCH_NAME)
        {
        } else {
            println!(
                "skipping 'check_toolchain_version_on_dev' as none of {}, {} and {} are set to {}",
                CRON_JOB_BRANCH_NAME_ENV_VAR,
                PR_TARGET_BRANCH_NAME_ENV_VAR,
                CI_BRANCH_NAME_ENV_VAR,
                TEST_BRANCH_NAME,
            );
            return;
        }

        let expected_toolchain_value = blocking::get(CASPER_NODE_TOOLCHAIN_URL)
            .expect("should get rust-toolchain from GitHub")
            .text()
            .expect("should parse rust-toolchain");

        // If this fails, ensure there's not a mismatch between ../resources/rust-toolchain.in and
        // https://github.com/casper-network/casper-node/blob/dev/smart_contracts/rust-toolchain.
        assert_eq!(&*expected_toolchain_value, CONTENTS);
    }
}
