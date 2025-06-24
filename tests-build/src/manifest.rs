use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Package {
    name: String,
    #[serde(default)]
    edition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Lib {
    harness: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(flatten)]
    pub rest: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Dependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(flatten)]
    pub rest: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BinOrTest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness: Option<bool>,
    #[serde(default)]
    pub path: PathBuf,
    #[serde(flatten)]
    pub rest: Map<String, Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Manifest {
    package: Option<Package>,
    dependencies: Map<String, Dependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lib: Option<Lib>,
    #[serde(rename = "bin", default, skip_serializing_if = "Vec::is_empty")]
    bins: Vec<BinOrTest>,
    #[serde(rename = "test", default, skip_serializing_if = "Vec::is_empty")]
    tests: Vec<BinOrTest>,
    #[serde(flatten)]
    rest: Map<String, Value>,
}

#[derive(Debug)]
pub(crate) enum TargetType {
    Bin(String),
    Lib,
    Test(String),
}

impl Manifest {
    // extract a manifest from a block quote like the following.
    // Must be wrapped in a multiline comment for now
    //
    // ```cargo
    // embedded-test = "*"
    // ```

    pub fn from_rust_file(test_file: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(test_file)?;

        let mut in_blockquote = false;
        let mut manifest = String::new();

        for line in content.lines() {
            if line.trim().starts_with("```cargo") {
                in_blockquote = true;
                continue;
            }
            if in_blockquote {
                if line.trim() == "```" {
                    break; // end of blockquote
                }
                manifest.push_str(line);
                manifest.push('\n');
            }
        }

        if manifest.trim().is_empty() {
            Err(anyhow!(
                "Manifest block not found in {}",
                test_file.display()
            ))
        } else {
            Ok(toml::from_str(&manifest)?)
        }
    }

    pub fn target_type(&self) -> anyhow::Result<TargetType> {
        if let Some(lib) = &self.lib {
            if lib.harness == Some(false) {
                return Ok(TargetType::Lib);
            }
            bail!("Lib with harness!=false is not supported");
        } else if let Some(bin) = self.bins.first() {
            if bin.harness == Some(false) {
                return Ok(TargetType::Bin(bin.name.clone()));
            }
            bail!("Bin with harness!=false is not supported");
        } else if let Some(test) = self.tests.first() {
            if test.harness == Some(false) {
                return Ok(TargetType::Test(test.name.clone()));
            }
            bail!("Test with harness!=false is not supported");
        } else {
            bail!("No lib, bin or test section found in manifest");
        }
    }

    pub fn fix(&mut self, test_file: &Path) -> anyhow::Result<()> {
        let directory = test_file
            .parent()
            .ok_or_else(|| anyhow!("Test file has no parent directory"))?;

        // Ensure the package name is set
        if self
            .package
            .as_ref()
            .map(|p| p.name.is_empty() || p.edition.is_empty())
            .unwrap_or(true)
        {
            self.package = Some(Package {
                name: "embedded-test".to_string(),
                edition: "2024".to_string(),
            });
        }

        // Normalize all dependency paths
        for dependency in self.dependencies.values_mut() {
            if let Some(path) = dependency.path.as_mut() {
                if path.is_relative() {
                    *path = directory.join(&path).canonicalize()?;
                }
            }
        }

        // Set the path to source file
        let target_type = self.target_type()?;
        match target_type {
            TargetType::Bin(name) => {
                let bin = self.bins.iter_mut().find(|bin| bin.name == name).unwrap();
                bin.path = test_file.into();
            }
            TargetType::Test(name) => {
                let test = self
                    .tests
                    .iter_mut()
                    .find(|test| test.name == name)
                    .unwrap();
                test.path = test_file.into();
            }
            TargetType::Lib => {
                if let Some(lib) = &mut self.lib {
                    lib.path = Some(test_file.into());
                } else {
                    unreachable!()
                }
            }
        }

        Ok(())
    }

    pub fn write(&self, manifest_path: &Path) -> anyhow::Result<()> {
        let manifest_str = toml::to_string_pretty(self)?;
        std::fs::write(manifest_path, manifest_str)?;
        Ok(())
    }
}
