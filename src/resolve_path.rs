#![allow(unused)]
use std::{fs, env};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
    pub fn new(
        project_path: PathBuf, 
        xmind_path: PathBuf,
    ) -> AllPath {
        let zip_path = xmind_path.with_extension("zip");
        let xlsx_filename = xmind_path.with_extension("xlsx").file_name().unwrap().to_owned();
        let xlsx_path = project_path.join("output").join(xlsx_filename);
        
        AllPath {
            project_path: project_path.clone(),
            input_dir_path: project_path.join("input"),
            output_dir_path: project_path.join("output"),
            xmind_path,
            zip_path,
            content_path: PathBuf::new(),
            xlsx_tmp_path: project_path.join("template").join("template.xlsx"),
            xlsx_path,
        }
    }

    pub fn set_allpath(xmind_path: PathBuf) -> AllPath {
        let project_path = env::current_dir().expect("Failed to get current directory");
        let input_dir_path = project_path.join("input");

        // 如果 xmind_path 不是绝对路径，则在 input 目录中查找
        let xmind_path = if xmind_path.is_absolute() {
            xmind_path
        } else {
            match find_file_in_dir(&input_dir_path, &xmind_path) {
                Some(p) => p,
                None => panic!("文件 {:?} 在 input 目录中未找到。", xmind_path),
            }
        };

        AllPath::new(project_path, xmind_path)
    }

    // Getter methods
    pub fn project_path(&self) -> &PathBuf { &self.project_path }
    pub fn input_dir_path(&self) -> &PathBuf { &self.input_dir_path }
    pub fn output_dir_path(&self) -> &PathBuf { &self.output_dir_path }
    pub fn xmind_path(&self) -> &PathBuf { &self.xmind_path }
    pub fn zip_path(&self) -> &PathBuf { &self.zip_path }
    pub fn content_path(&self) -> &PathBuf { &self.content_path }
    pub fn xlsx_tmp_path(&self) -> &PathBuf { &self.xlsx_tmp_path }
    pub fn xlsx_path(&self) -> &PathBuf { &self.xlsx_path }

    // Setter methods
    pub fn change_xmind_path(&mut self, new_path: PathBuf) {
        self.xmind_path = new_path;
    }

    pub fn change_content_path(&mut self, new_path: PathBuf) {
        self.content_path = new_path;
    }
}

pub fn get_xmind_path(path: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
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

// 在目录中查找文件
pub fn find_file_in_dir(dir: &PathBuf, file_name: &PathBuf) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let entry_path = entry.path();

        if entry_path.is_file() && entry_path.file_name() == file_name.file_name() {
            return Some(entry_path);
        }
    }
    None
}