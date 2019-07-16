use std::fmt;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::error::Error;

use crate::options::Options;

#[derive(Default)]
pub struct LineCounter {
    line_count: usize,
    current_path: PathBuf,
    ops: Options,
}

impl LineCounter {
    pub fn new(ops: Options) -> LineCounter {
        LineCounter {
            current_path: ops.dir.clone(),
            ops,
            ..Default::default()
        }
    }

    pub fn get_line_count(&mut self) -> io::Result<()> {
        // If there was an error, if it was a permission erorr then just tell
        // the user and contine.
        let curr_path = self.current_path.as_path();
        let metadata = fs::symlink_metadata(curr_path)?;
        let f_type = metadata.file_type();

        if f_type.is_dir() && self.ops.recursive {
            match self.current_path.read_dir() {
                Ok(read_dir) => {
                    for sub in read_dir {
                        self.current_path = sub?.path();
                        self.get_line_count()?;
                    }
                },
                Err(e) => {
                    if e.kind() == ErrorKind::PermissionDenied {
                        // Do not want to return from function if it is just a permission error.
                        eprintln!("linecount: {}: Permission denied.", self.current_path.display());
                    } else {
                        return Err(e)
                    }
                }
            }
        } else if !f_type.is_symlink() && !f_type.is_dir() {
            if self.ops.file_extensions.len() > 0 {
                if let Some(extension) = curr_path.extension() {
                    if self.ops.file_extensions.contains(&extension.to_os_string()) {
                        self.line_count += Self::get_file_line_count(curr_path)?;
                    }
                }
            } else {
                self.line_count += Self::get_file_line_count(curr_path)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn get_file_line_count(file_path: &Path) -> io::Result<usize> {
        match fs::read_to_string(file_path) {
            Ok(file_string) => {
                Ok(file_string.lines().count())
            },
            Err(e) => {
                if e.kind() == ErrorKind::InvalidData {
                    eprintln!("linecounter: read error with file: {}: {}", file_path.display(), e.description());
                    Ok(0)
                } else {
                    Err(e)
                }
            }
        }
    }
}

// For outputting result.
impl fmt::Display for LineCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line count: {}", self.line_count)
    }
}