use std::path::Path;
use std::path::PathBuf;
use std::fs;

use std::io;
use std::io::ErrorKind;

/// Helper for creating and deleting project build directory
///
/// # Examples
///
/// ```
/// use dml::helpers::BuildDir;
///
/// // Creating a new BuildDir automatically create a `build` directory in
/// // curent working directory
/// let dir = BuildDir::new().unwrap();
///
/// //let a_file_path = dir.file("myfile");
///
/// // Delete the build directory
/// dir.delete().unwrap();
/// ```
pub struct BuildDir {
    path: PathBuf,
}

impl BuildDir {
    pub fn new() -> io::Result<BuildDir> {
        let path = Path::new("build/").to_path_buf();
        let build_dir = BuildDir {path:path.clone()};
        match fs::create_dir(&path) {
            Ok(_)   => Ok(build_dir),
            Err(e)  => {
                match e.kind() {
                    ErrorKind::AlreadyExists    => Ok(build_dir),
                    _                           => Err(e),
                }
            }
        }
    }

    pub fn delete(self) -> io::Result<()> {
        fs::remove_dir_all(self.path)
    }
}
