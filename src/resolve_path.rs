#![allow(unused)]
use std::{ io, path::Path, path::PathBuf };
use std::fs::{ self, File };

#[derive(Debug)]
pub struct AllPath {
    project_path: PathBuf,
    input_dir_path: PathBuf,
    output_dir_path: PathBuf,
    xmind_path: PathBuf,
    zip_path: PathBuf,
    content_path: PathBuf,
    xlsx_tmp_path: PathBuf,
    xlsx_path: PathBuf,
}

impl AllPath {
    pub fn new(project_path: &str, input_dir_path: PathBuf, xmind_path: PathBuf, zip_path: PathBuf, xlsx_path: PathBuf) -> AllPath {
        AllPath {
            project_path: PathBuf::from(project_path),
            input_dir_path,
            output_dir_path: PathBuf::from(format!("{}{}", project_path,"/output")),
            xmind_path,
            zip_path,
            content_path: PathBuf::new(),
            xlsx_tmp_path: PathBuf::from(format!("{}{}", project_path,"/tmplate/tmplate.xlsx")),
            xlsx_path,
        }
    }

    pub fn project_path(&self) -> &PathBuf {
        &self.project_path
    }

    pub fn input_dir_path(&self) -> &PathBuf {
        &self.input_dir_path
    }

    pub fn output_dir_path(&self) -> &PathBuf {
        &self.output_dir_path
    }

    pub fn xmind_path(&self) -> &PathBuf {
        &self.xmind_path
    }
    
    pub fn zip_path(&self) -> &PathBuf {
        &self.zip_path
    }

    pub fn content_path(&self) -> &PathBuf {
        &self.content_path
    }

    pub fn change_xmind_path(&mut self, new_path: PathBuf) {
        self.xmind_path = new_path;
    }

    pub fn change_content_path(&mut self, new_path: PathBuf) {
        self.content_path = new_path;
    }

    pub fn xlsx_path(&self) -> &PathBuf {
        &self.xlsx_path
    }

    pub fn xlsx_tmp_path(&self) -> &PathBuf {
        &self.xlsx_tmp_path
    }
}

pub fn get_xmind_path(path: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
    // 遍历input目录，取第一个路径
    let mut xmind_paths = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            println!("File: {}", entry_path.display());
            xmind_paths.push(entry_path);
        } else if entry_path.is_dir() {
            println!("Directory: {}", entry_path.display());
        }
    }
    Ok(xmind_paths)
}