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

        "#;

        std::fs::write(manifest_path, manifest_str)?;
        Ok(())
    }
}
