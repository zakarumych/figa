//! Provides very opinionated loader for config from predefined locations.

use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;

use crate::Figa;

/// Load config from a file.
pub fn update_from_path<T>(config: &mut T, mut path: PathBuf) -> io::Result<()>
where
    T: Figa,
{
    let cfg_string = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(err) if path.extension().is_none() && err.kind() == io::ErrorKind::NotFound => {
            path.set_extension("toml");
            fs::read_to_string(&path)?
        }
        Err(err) => return Err(err),
    };

    config
        .update(toml::de::Deserializer::new(&cfg_string))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

/// Updates configuration from predefined directories and current working directory.
pub fn update_config<T>(config: &mut T, dirs: &ProjectDirs, name: &str) -> io::Result<()>
where
    T: Figa,
{
    update_from_path(config, dirs.config_dir().join(name))?;
    update_from_path(config, dirs.config_local_dir().join(name))?;
    if let Ok(cd) = std::env::current_dir() {
        update_from_path(config, cd.join(name))?;
    }

    Ok(())
}

/// Loads configuration from predefined directories and current working directory.
pub fn load_config<T>(dirs: &ProjectDirs, name: &str) -> io::Result<T>
where
    T: Figa + Default,
{
    let mut config = T::default();
    update_config(&mut config, dirs, name)?;
    Ok(config)
}
