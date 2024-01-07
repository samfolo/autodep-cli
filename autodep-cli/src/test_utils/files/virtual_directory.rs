extern crate tempfile;

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::PathBuf;
use tempfile::{Builder, NamedTempFile, TempDir};

pub struct VirtualDirectory {
    base_dir: Option<TempDir>,
    default_extension: Option<String>,
}

impl VirtualDirectory {
    pub fn new(default_extension: Option<&str>) -> io::Result<Self> {
        let base_dir = TempDir::new()?;
        let base_dir = Some(base_dir);

        Ok(Self {
            base_dir,
            default_extension: default_extension.map(String::from),
        })
    }

    pub fn base_dir(&self) -> &TempDir {
        if let Some(base_dir) = &self.base_dir {
            base_dir
        } else {
            panic!("VirtualDirectory has been closed");
        }
    }

    pub fn cwd(&self) -> io::Result<PathBuf> {
        self.base_dir().path().canonicalize()
    }

    pub fn create_file(
        &self,
        relative_dir: &str,
        name: &str,
        extension: Option<&OsStr>,
        content: Option<&str>,
    ) -> io::Result<NamedTempFile> {
        let suffix = if let Some(ext) = extension {
            format!(".{}", ext.to_str().unwrap())
        } else {
            self.default_extension
                .as_ref()
                .map(|ext| format!(".{}", ext))
                .unwrap_or_default()
        };

        let mut builder = Builder::new()
            .prefix(&name)
            .suffix(&suffix)
            .rand_bytes(0)
            .tempfile_in(self.resolve_path(relative_dir)?)?;

        if let Some(content) = content {
            writeln!(builder, "{}", content)?;
        }

        Ok(builder)
    }

    pub fn create_subdir(&self, relative_dir: &str) -> io::Result<TempDir> {
        Builder::new().tempdir_in(&self.base_dir().path().join(relative_dir).canonicalize()?)
    }

    fn resolve_path(&self, relative_path: &str) -> io::Result<PathBuf> {
        let mut path = self.base_dir().path().to_path_buf();
        path.push(relative_path);
        path = path.canonicalize()?;
        Ok(path)
    }
}
