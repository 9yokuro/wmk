use std::{env, fs, io, path};

pub fn xdg_data_home() -> path::PathBuf {
    match env::var("XDG_DATA_HOME") {
        Ok(x) => path::Path::new(&x).to_path_buf(),
        Err(_) => path::Path::new(&env::var("HOME").unwrap())
            .join(".local/share")
            .to_path_buf(),
    }
}

pub fn create_file_all<P: AsRef<path::Path>>(path: P) -> io::Result<()> {
    let parent_dir = path.as_ref().parent().unwrap();
    fs::create_dir_all(parent_dir)?;
    fs::File::create(path)?;
    Ok(())
}
