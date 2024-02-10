extern crate tempfile;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::{Builder, NamedTempFile, TempDir};

pub enum ItemConfig {
    File {
        name: String,
        extension: Option<String>,
        content: Option<String>,
    },
    Directory {
        name: String,
        items: Vec<ItemConfig>,
    },
}

pub struct TreeConfig {
    pub items: Vec<ItemConfig>,
}

pub struct VirtualDirectory {
    base_dir: Option<TempDir>,
    default_extension: Option<String>,
}

impl VirtualDirectory {
    pub fn derive_path(dir: &TempDir, parts: Vec<&str>) -> String {
        let mut path = dir.path().to_path_buf();
        for part in parts {
            path = path.join(part);
        }

        path.canonicalize().unwrap().to_str().unwrap().to_string()
    }
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

    pub fn from(config: &TreeConfig) -> io::Result<Self> {
        let base_dir = TempDir::new()?;
        let base_dir = Some(base_dir);

        let vd = Self {
            base_dir,
            default_extension: None,
        };

        let base_dirname = &vd.base_dir().path().to_str();
        let base_dirname = if let Some(name) = base_dirname {
            name
        } else {
            panic!("Invalid base directory name");
        };

        vd.create_tree(base_dirname, config)?;
        Ok(vd)
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
    ) -> io::Result<String> {
        let suffix = if let Some(ext) = extension {
            format!(".{}", ext.to_str().unwrap_or_default())
        } else if let Some(ext) = &self.default_extension {
            format!(".{}", ext)
        } else {
            String::new()
        };

        let file_name = format!("{}{}", name, suffix);
        let file_path = Path::new(relative_dir).join(file_name);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(&file_path)?;

        if let Some(content) = content {
            writeln!(file, "{}", content)?;
        }

        Ok(file_path.to_string_lossy().into_owned())
    }

    pub fn create_subdir(&self, relative_dir: &str) -> io::Result<String> {
        let subdirname = &self.base_dir().path().join(relative_dir);
        fs::create_dir(subdirname)?;
        Ok(subdirname.to_str().unwrap().to_string())
    }

    pub fn create_tree(&self, relative_dir: &str, config: &TreeConfig) -> io::Result<&TempDir> {
        self.create_items(relative_dir, &config.items)?;
        Ok(self.base_dir())
    }

    fn create_items(&self, relative_dir: &str, items: &[ItemConfig]) -> io::Result<()> {
        for item in items {
            match item {
                ItemConfig::File {
                    name,
                    extension,
                    content,
                } => {
                    let ext = extension.as_deref().map(OsStr::new);
                    let _ = self.create_file(relative_dir, name, ext, content.as_deref())?;
                }
                ItemConfig::Directory {
                    name,
                    items: sub_items,
                } => {
                    let _ = self.create_subdir(name)?;
                    let subdirname = PathBuf::from(relative_dir).join(name);
                    self.create_items(&subdirname.to_str().unwrap(), sub_items)?;
                }
            }
        }
        Ok(())
    }

    pub fn close(&mut self) {
        if let Some(base_dir) = self.base_dir.take() {
            let _ = base_dir.close();
        }
    }
}
