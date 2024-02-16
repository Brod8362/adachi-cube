use std::{path::{Path, PathBuf}, error::Error};

use thiserror::Error;


pub struct Folders {
    true_path: PathBuf,
    maybe_path: PathBuf,
    false_path: PathBuf
}

#[derive(Error, Debug)]
enum FolderPathError {
    #[error("`{0}` is not a directory")]
    NotADirectory(String),
    #[error("`{0}` has no chilren")]
    NoFilesInDirectory(String)
}

impl Folders {
    pub fn new(true_path_str: &String, maybe_path_str: &String, false_path_str: &String) -> Result<Folders, Box<dyn Error>> {
        let true_path = PathBuf::from(true_path_str);
        let maybe_path = PathBuf::from(maybe_path_str);
        let false_path = PathBuf::from(false_path_str);
        let paths_to_check = [&true_path, &maybe_path, &false_path];
        for path in paths_to_check {
            if !path.is_dir() {
                return Err(
                    Box::new(
                        FolderPathError::NotADirectory(path.to_str().unwrap().to_owned()
                    )
                ));
            }
        }
        Ok(Self {
            true_path,
            maybe_path,
            false_path
        })
    }

    pub fn pick_random(&self) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        //return random file
        let num: i32 = rand::random();
        let folder = match (num % 3).abs() {
            0 => &self.true_path,
            1 => &self.maybe_path,
            2 => &self.false_path,
            _ => panic!("math is broken!")
        };
        let children: Vec<PathBuf> = folder.read_dir()?
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap().path())
            .collect();
        if children.is_empty() {
            //this shouldn't be able to happen anyway, but in case it changes during runtime
            return Err(
                Box::new(
                    FolderPathError::NotADirectory(folder.to_str().unwrap().to_owned()
                )
            ));
        }
        let index: usize = rand::random::<usize>() % children.len();
        Ok(children[index].clone())
    }
}