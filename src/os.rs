use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_search_path() -> Result<Vec<String>, std::env::VarError> {
    let var = env::var("PATH")?;
    Ok(var.split(":").map(|x| x.to_string()).collect::<Vec<_>>())
}

pub fn get_working_directory() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn change_working_directory(path: &str) -> std::io::Result<()> {
    let root = Path::new(path);
    env::set_current_dir(&root)
}

pub fn find_file<T: AsRef<str>>(name: &str, path_list: &[T]) -> Vec<PathBuf> {
    path_list
        .iter()
        .map(|s| std::path::Path::new(s.as_ref()))
        .map(|p| p.join(name))
        .filter(|p| p.exists())
        .collect::<Vec<_>>()
}
