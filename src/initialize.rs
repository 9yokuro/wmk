use filey::{Filey, Result};
use std::path::Path;

pub fn initialize<P: AsRef<Path>>(path: P) -> Result<()> {
    let filey = Filey::new(path);

    if !filey.exists() {
        filey.create_dir()?;
    }

    Ok(())
}
