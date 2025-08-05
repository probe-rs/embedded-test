use std::path::Path;

pub(crate) struct BuildRs {
    linker_files: Vec<String>,
}

impl BuildRs {
    pub fn new() -> Self {
        Self {
            linker_files: Vec::new(),
        }
    }

    pub fn add_linker_file(&mut self, file: &str) {
        self.linker_files.push(file.to_string());
    }

    pub fn write(&self, build_rs_path: &Path) -> anyhow::Result<()> {
        let statements = self.generate_linker_args();
        let contents = format!("fn main() {{ {statements} }}");
        std::fs::write(build_rs_path, contents)?;
        Ok(())
    }

    pub fn generate_linker_args(&self) -> String {
        self.linker_files
            .iter()
            .map(|file| format!("cargo:rustc-link-arg=-T{}", file))
            .map(|arg| format!("println!(\"{}\");\n", arg))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
