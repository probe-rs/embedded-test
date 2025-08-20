use std::path::Path;

pub struct WorkspaceManifest {}

impl WorkspaceManifest {
    pub fn new() -> Self {
        WorkspaceManifest {}
    }

    pub fn write(&self, manifest_path: &Path) -> anyhow::Result<()> {
        let manifest_str = r#"

        [workspace]
        resolver = "3"
        members = ["cases/*"]

        # For now we have to use future versions
        [patch.crates-io]
        esp-hal = { git = "https://github.com/bugadani/esp-hal", branch = "executor" }
        esp-bootloader-esp-idf = { git = "https://github.com/bugadani/esp-hal", branch = "executor" }
        esp-hal-embassy = { git = "https://github.com/bugadani/esp-hal", branch = "executor" }
        esp-println = { git = "https://github.com/bugadani/esp-hal", branch = "executor" }
        esp-backtrace = { git = "https://github.com/bugadani/esp-hal", branch = "executor" }

        "#;

        std::fs::write(manifest_path, manifest_str)?;
        Ok(())
    }
}
