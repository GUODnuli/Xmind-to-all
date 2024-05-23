#![allow(unused)]
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{ env, fmt::format };
use tokio::fs as tokio_fs;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::signal;
use zip::ZipArchive;
use gtk::prelude::*;
use gtk::{ glib, Application, ApplicationWindow, FileChooserAction, FileChooserButton, Orientation, Box as GtkBox, Label};

mod json_to_sheet;
use json_to_sheet::{Sheet, Topic};

mod sheet_to_tree;
use sheet_to_tree::TestTree;

mod unzip;

mod resolve_path;
use resolve_path::AllPath;

enum Event {
    ProcessXmind,
    Exit,
}

const APP_ID: &str = "org.gtk_rs.xmind_to_all";

#[tokio::main]
async fn main() -> glib::ExitCode {
    let(tx, mut rx) = mpsc::channel(32);

    let app = Application::new(Some(APP_ID), Default::default());

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("XMind to All"));
        window.set_default_size(800, 600);

        let vbox = GtkBox::new(Orientation::Vertical, 5);

        let label = Label::new(Some("Choose a file:"));
        vbox.append(&label);

        let file_chooser_button = FileChooserButton::new(Some("Select a File"), FileChooserAction::Open);
        vbox.append(&file_chooser_button);

        file_chooser_button.connect_file_set(move |file_chooser| {
            if let Some(file) = file_chooser.file() {
                println!("File selected: {:?}", file);
            }
        });

        window.set_child(Some(&vbox));
        window.show();
    });

    // 启动事件处理任务
    let event_handler = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                Event::ProcessXmind => {
                    process_xmind().await;
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
            match line.trim() {
                "process" => {
                    tx_clone.send(Event::ProcessXmind).await.unwrap();
                },
                "exit" => {
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

    app.run()
}

async fn process_xmind() {
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
    tokio_fs::copy(path_value.xmind_path(), path_value.zip_path()).await.expect("复制xmind为zip时遇到不可恢复的问题。");
    let mut content_path = unzip::extract_zip(path_value.zip_path())
        .unwrap_or_else(|err| {
            panic!("zip解压时遇到无法恢复的问题。")
        });
    content_path.push("content.json");
    tokio_fs::remove_file(path_value.zip_path()).await.expect("移除压缩包时遇到不可预期的问题。");
    AllPath::change_content_path(&mut path_value, content_path);

    // 获取content.json数据
    let content_path = path_value.content_path();
    let contents = json_to_sheet::get_sheet_json(content_path).expect("获取内容时遇到无法解决的问题。");

    // 创建测试用例树
    let mut testlist_tree = TestTree::new();
    testlist_tree.create_testtree(&contents);
}
