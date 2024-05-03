use std::{env, fs, io, path};
use walkdir::WalkDir;

pub fn xdg_data_home() -> path::PathBuf {
    match env::var("XDG_DATA_HOME") {
        Ok(x) => path::Path::new(&x).to_path_buf(),
        Err(_) => path::Path::new(&env::var("HOME").unwrap())
            .join(".local")
            .join("share")
            .to_path_buf(),
    }
}

pub fn create_file_all<P>(path: P) -> io::Result<()>
where
    P: AsRef<path::Path>,
{
    let parent_dir = path.as_ref().parent().unwrap();
    fs::create_dir_all(parent_dir)?;
    fs::File::create(path)?;
    Ok(())
}

pub fn is_empty_dir<P>(path: P) -> bool
where
    P: AsRef<path::Path>,
{
    WalkDir::new(path).into_iter().nth(1).is_none()
}
