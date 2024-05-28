use std::path::PathBuf;
use std::env;
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

use xmind_to_all::json_to_sheet;
use xmind_to_all::sheet_to_tree::TestcaseTree;
use xmind_to_all::resolve_path::AllPath;
use xmind_to_all::unzip;
use xmind_to_all::write_to_xlsx::write_xlsx;
use xmind_to_all::user_config;

enum Event {
    ProcessXmind(PathBuf),
    AddConfig(&str, &str),
    Exit,
}

#[tokio::main]
async fn main() {
    let(tx, mut rx) = mpsc::channel(32);

    // 启动事件处理任务
    let event_handler = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                Event::ProcessXmind(path) => {
                    process_xmind(path.to_str().unwrap()).await;
                },
                Event::AddConfig(key, value) => {
                    user_config::add_config(key, value);
                },
                Event::Exit => {
                    println!("Exiting...");
                    break;
                },
            }
        }
    });

    let tx_clone = tx.clone();
    let command_handler = tokio::spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            let mut parts = line.trim().split_whitespace();
            match parts.next() {
                Some("process") => {
                    if let Some(path) = parts.next() {
                        let path = PathBuf::from(path);
                        tx_clone.send(Event::ProcessXmind(path)).await.unwrap();
                    } else {
                        println!("Missing path.");
                    }
                },
                Some("add_config") => {
                    if let Some(param) = parts.next() {
                        let (key, value) = param.split_once("=").unwrap();
                        tx_clone.send(Event::AddConfig(key, value)).await.unwrap();
                    }
                },
                Some("exit") => {
                    tx_clone.send(Event::Exit).await.unwrap();
                },
                _ => {
                    println!("Unknown command.");
                }
            }
        }
    });

    // 等待任务完成
    tokio::select! {
        _ = event_handler => {},
        _ = command_handler => {},
    }
}

async fn process_xmind(xmind_file_path: &str) {
    // 初始化项目路径与input目录变量
    let project_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_dir_path = PathBuf::from(format!("{}/{}", project_path, "input"));
    
    // 构造解压文件路径并加入到路径结构体中
    // let zip_path = PathBuf::from(&xmind_path.with_extension("zip"));
    let xmind_path = PathBuf::from(xmind_file_path);
    let zip_path = PathBuf::from(xmind_path.with_extension("zip"));
    let file_name = xmind_path.file_stem().unwrap().to_str().unwrap();
    let xlsx_path = PathBuf::from(format!("{}{}", project_path, format!("/output/{}.xlsx", file_name)));

    // 初始化路径结构体
    let mut path_value = AllPath::new(&project_path, input_dir_path, xmind_path, zip_path, xlsx_path);
    
    // copy一份xmind为zip文件并解压，并返回content.json文件的路径
    fs::copy(path_value.xmind_path(), path_value.zip_path()).await.expect("复制xmind为zip时遇到无法解决的问题。");
    let mut content_path = unzip::extract_zip(path_value.zip_path())
        .unwrap_or_else(|err| {
            panic!("zip解压时遇到无法解决的问题。")
        });
    content_path.push("content.json");
    fs::remove_file(path_value.zip_path()).await.expect("移除压缩包时遇到无法解决的问题。");
    AllPath::change_content_path(&mut path_value, content_path);

    // 获取content.json数据
    let content_path = path_value.content_path();
    let contents = json_to_sheet::get_sheet_json(content_path).expect("获取内容时遇到无法解决的问题。");

    // 创建测试用例树
    let mut testcase_tree = TestcaseTree::from(&contents);

    println!("{:?}", &path_value.xlsx_tmp_path());
    fs::copy(path_value.xlsx_tmp_path(), path_value.xlsx_path()).await.expect("复制xlsx模板时遇到无法解决的问题。");

    let user_config = path_value.user_config();
    write_xlsx(testcase_tree, path_value.xlsx_path().to_str().unwrap());

    println!("处理完成。");
}
