use std::{fs, path::Path, process, str};

use colour::e_red;
use once_cell::sync::Lazy;

use crate::{dependency::Dependency, CasperOverrides, ARGS, FAILURE_EXIT_CODE};

pub static CL_CONTRACT: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-contract", "1.4.2"));
pub static CL_TYPES: Lazy<Dependency> = Lazy::new(|| Dependency::new("casper-types", "1.4.4"));
pub static CL_ENGINE_TEST_SUPPORT: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-engine-test-support", "2.0.2"));
pub static CL_EXECUTION_ENGINE: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-execution-engine", "1.4.2"));
pub static PATCH_SECTION: Lazy<String> = Lazy::new(|| match ARGS.casper_overrides() {
    Some(CasperOverrides::WorkspacePath(path)) => {
        format!(
            r#"[patch.crates-io]
casper-contract = {{ path = "{0}/smart_contracts/contract" }}
casper-engine-test-support = {{ path = "{0}/execution_engine_testing/test_support" }}
casper-execution-engine = {{ path = "{0}/execution_engine" }}
casper-types = {{ path = "{0}/types" }}
"#,
            path.display()
        )
    }
    Some(CasperOverrides::GitRepo { url, branch }) => {
        format!(
            r#"[patch.crates-io]
casper-contract = {{ git = "{0}", branch = "{1}" }}
casper-engine-test-support = {{ git = "{0}", branch = "{1}" }}
casper-execution-engine = {{ git = "{0}", branch = "{1}" }}
casper-types = {{ git = "{0}", branch = "{1}" }}
"#,
            url, branch
        )
    }
    None => String::new(),
});

pub fn print_error_and_exit(msg: &str) -> ! {
    e_red!("error");
    eprintln!("{}", msg);
    process::exit(FAILURE_EXIT_CODE)
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) {
    if let Err(error) = fs::create_dir_all(path.as_ref()) {
        print_error_and_exit(&format!(
            ": failed to create '{}': {}",
            path.as_ref().display(),
            error
        ));
    }
}

pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
    if let Err(error) = fs::write(path.as_ref(), contents) {
        print_error_and_exit(&format!(
            ": failed to write to '{}': {}",
            path.as_ref().display(),
            error
        ));
    }
}

#[cfg(test)]
pub mod tests {
    use reqwest::blocking;
    use toml::Value;

    use super::*;

    const CL_CONTRACT_TOML_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/smart_contracts/contract/Cargo.toml";
    const CL_TYPES_TOML_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/types/Cargo.toml";
    const CL_ENGINE_TEST_SUPPORT_TOML_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/execution_engine_testing/test_support/Cargo.toml";
    const CL_EXECUTION_ENGINE_TOML_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/execution_engine/Cargo.toml";
    const PACKAGE_FIELD_NAME: &str = "package";
    const VERSION_FIELD_NAME: &str = "version";

    /// Checks the version of the package specified by the Cargo.toml at `toml_path` is equal to
    /// the hard-coded one specified in `dep.version()`.
    pub fn check_package_version(dep: &Dependency, toml_url: &str) {
        let toml_contents = blocking::get(toml_url)
            .unwrap_or_else(|error| {
                panic!(
                    "should get Cargo.toml for {} from GitHub: {}",
                    dep.name(),
                    error
                )
            })
            .text()
            .unwrap_or_else(|error| {
                panic!("should parse Cargo.toml for {}: {}", dep.name(), error)
            });

        let toml = toml_contents.parse::<Value>().unwrap();

        let expected_version = toml[PACKAGE_FIELD_NAME][VERSION_FIELD_NAME]
            .as_str()
            .unwrap();
        // If this fails, ensure `dep.version()` is updated to match the value in the Cargo.toml at
        // `toml_url`.
        assert_eq!(
            expected_version,
            dep.version(),
            "\n\nEnsure local version of {:?} is updated to {} as defined in {}\n\n",
            dep,
            expected_version,
            toml_url
        );
    }

    #[test]
    fn check_cl_contract_version() {
        check_package_version(&*CL_CONTRACT, CL_CONTRACT_TOML_URL);
    }

    #[test]
    #[ignore]
    fn check_cl_types_version() {
        check_package_version(&*CL_TYPES, CL_TYPES_TOML_URL);
    }

    #[test]
    #[ignore]
    fn check_cl_engine_test_support_version() {
        check_package_version(&*CL_ENGINE_TEST_SUPPORT, CL_ENGINE_TEST_SUPPORT_TOML_URL);
    }

    #[test]
    fn check_cl_execution_engine_version() {
        check_package_version(&*CL_EXECUTION_ENGINE, CL_EXECUTION_ENGINE_TOML_URL);
    }
}
