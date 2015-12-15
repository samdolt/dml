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
    /// Create a new BuildDir in ./build
    pub fn new() -> io::Result<BuildDir> {
        BuildDir::with_path("build/")
    }

    /// Create a new BuildDir at the given path
    pub fn with_path<P: AsRef<Path>>(path: P) -> io::Result<BuildDir> {
        let path = path.as_ref().to_path_buf();
        let build_dir = BuildDir { path: path.clone() };
        match fs::create_dir(&path) {
            Ok(_) => Ok(build_dir),
            Err(e) => {
                match e.kind() {
                    ErrorKind::AlreadyExists => Ok(build_dir),
                    _ => Err(e),
                }
            }
        }
    }

    /// Create a new BuildDir in a subfolder of current Buildir
    ///
    /// # Example
    /// ```
    /// use dml::helpers::BuildDir;
    ///
    /// // New build dir at ./build/
    /// let build = BuildDir::new().unwrap();
    ///
    /// // New build dir at ./build/html/
    /// let html = build.dir("html").unwrap();
    /// ```
    pub fn dir<P: AsRef<Path>>(&self, path: P) -> io::Result<BuildDir> {
        let mut builder = self.path.clone();
        builder.push(path);

        BuildDir::with_path(&builder.as_path())
    }

    pub fn delete(self) -> io::Result<()> {
        fs::remove_dir_all(self.path)
    }
}
