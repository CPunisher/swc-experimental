use std::{
    env,
    fs::read_dir,
    io::{self, ErrorKind},
    path::PathBuf,
};

pub fn crate_root() -> PathBuf {
    workspace_root().unwrap().join("tasks").join("testsuite")
}

pub fn workspace_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == "Cargo.lock");
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
