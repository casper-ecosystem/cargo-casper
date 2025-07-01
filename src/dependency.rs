use crate::ARGS;

/// Used to hold the information about the Casper dependencies which will be required by the
/// generated Cargo.toml files.
#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: String,
    /// This flag will force a dependency to be always used with it's declared `version` (for
    /// some tests we subtitute version with `*`). Some dependencies need to be set in a very
    /// specific version, because their edition configurations in newer patch versions are
    /// not compatible with our rust toolchain
    allow_asterixify: bool,
}

impl Dependency {
    pub fn new(name: &str, version: &str) -> Self {
        Dependency {
            name: name.to_string(),
            version: version.to_string(),
            allow_asterixify: true,
        }
    }

    pub fn new_disallow_asterix(name: &str, version: &str) -> Self {
        Dependency {
            name: name.to_string(),
            version: version.to_string(),
            allow_asterixify: false,
        }
    }

    pub fn display_with_features(&self, default_features: bool, features: Vec<&str>) -> String {
        let version = if self.allow_asterixify && ARGS.casper_overrides().is_some() {
            "*"
        } else {
            &self.version
        };

        if default_features && features.is_empty() {
            return format!("{} = \"{}\"\n", self.name, version);
        }

        let mut output = format!(r#"{} = {{ version = "{}""#, self.name, version);

        if !default_features {
            output = format!("{}, default-features = false", output);
        }

        if !features.is_empty() {
            output = format!("{}, features = {:?}", output, features);
        }

        format!("{} }}\n", output)
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(test)]
    pub fn version(&self) -> &str {
        &self.version
    }
}
