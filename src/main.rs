#![allow(unused)]
use std::ffi::OsStr;
use std::fs::{ self, File };
use std::io;
use std::path::PathBuf;
use std::{ env, fmt::format };

use zip::ZipArchive;

mod json_to_sheet;
use json_to_sheet::{Sheet, Topic};

mod sheet_to_tree;
use sheet_to_tree::TestTree;

mod unzip;

mod resolve_path;
use resolve_path::AllPath;

fn main() {
    // 初始化项目路径与input目录变量
    let project_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_dir_path = PathBuf::from(format!("{}/{}", project_path, "input"));

    // 获取xmind路径：input目录下遍历到的第一个文件
    let xmind_path = resolve_path::get_xmind_path(&input_dir_path)
        .and_then(|mut paths| paths.into_iter().next().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "没有找到xmind文件。")))
        .and_then(|path| {
            if path.extension() == Some(OsStr::new("xmind")) {
                Ok(path)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "找到的文件不是xmind格式。"))
            }
        })
        .unwrap_or_else(|e| panic!("{}", e));
    
    // 构造解压文件路径并加入到路径结构体中
    let zip_path = PathBuf::from(&xmind_path.with_extension("zip"));

    // 初始化路径结构体
    let mut path_value = AllPath::new(&project_path, input_dir_path, xmind_path, zip_path);
    
    
    // copy一份xmind为zip文件并解压，并返回content.json文件的路径
    fs::copy(path_value.xmind_path(), path_value.zip_path()).expect("复制xmind为zip时遇到不可恢复的问题。");
    let mut content_path = unzip::extract_zip(path_value.zip_path())
        .unwrap_or_else(|err| {
            panic!("zip解压时遇到无法恢复的问题。")
        });
    content_path.push("content.json");
    fs::remove_file(path_value.zip_path()).expect("移除压缩包时遇到不可预期的问题。");
    AllPath::change_content_path(&mut path_value, content_path);

    // 获取content.json数据
    let content_path = path_value.content_path();
    // println!("{}", contents);
    let contents = json_to_sheet::get_sheet_json(content_path).expect("获取内容时遇到无法解决的问题。");

    let mut testtree = TestTree::new();
    testtree.create_testtree(&contents);
}
