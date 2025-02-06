use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Stdio,
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

pub enum StandardIO {
    Default,
    File { path: String, append: bool },
}

impl StandardIO {
    pub fn try_stdout(&self) -> io::Result<Stdio> {
        match self {
            StandardIO::Default => Ok(io::stdout().into()),
            StandardIO::File { path, append } => make_stdio(&path, *append),
        }
    }

    pub fn try_stderr(&self) -> io::Result<Stdio> {
        match self {
            StandardIO::Default => Ok(io::stderr().into()),
            StandardIO::File { path, append } => make_stdio(&path, *append),
        }
    }

    pub fn try_stdout_write(&self) -> io::Result<Box<dyn Write>> {
        match self {
            StandardIO::Default => Ok(Box::new(io::stderr())),
            StandardIO::File { path, append } => make_write(path, *append),
        }
    }
}

fn make_stdio(path: &str, append: bool) -> io::Result<Stdio> {
    let file = open_file(path, append)?;
    Ok(Stdio::from(file))
}

fn make_write(path: &str, append: bool) -> io::Result<Box<dyn Write>> {
    let file = open_file(path, append)?;
    Ok(Box::new(file))
}

fn open_file(path: &str, append: bool) -> io::Result<File> {
    OpenOptions::new()
        .append(append)
        .write(true)
        .create(true)
        .open(path)
}
