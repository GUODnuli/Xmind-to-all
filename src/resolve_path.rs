#![allow(unused)]
use std::{ io, path::Path, path::PathBuf, env };
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
    pub fn new(
        current_path: PathBuf, 
        xmind_path: PathBuf, 
        zip_path: PathBuf, 
        xlsx_path: PathBuf
    ) -> AllPath {
        AllPath {
            project_path: current_path.clone(),
            input_dir_path: current_path.join("input"),
            output_dir_path: current_path.join("output"),
            xmind_path,
            zip_path,
            content_path: PathBuf::new(),
            xlsx_tmp_path: current_path.join("template").join("template.xlsx"),
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
    pub fn set_allpath(xmind_path: PathBuf) -> AllPath {
        let out_dir = env::var("OUT_DIR").unwrap();
        println!("OUT_DIR: {}", out_dir);
        let project_path = env::current_dir().expect("Failed to get current directory");
        let zip_path = PathBuf::from(xmind_path.with_extension("zip"));
        let xlsxfile_relative_path = xmind_path.clone().with_extension("xlsx").file_name().unwrap().to_owned();
        let xlsx_path = {
            #[cfg(target_os = "windows")]
            {
                project_path.join(xlsxfile_relative_path.to_str().unwrap().replace("/", "\\"))
            }

            #[cfg(not(target_os = "windows"))]
            {
                project_path.join(xlsxfile_relative_path)
            }
        };
        AllPath::new(project_path, xmind_path, zip_path, xlsx_path)
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