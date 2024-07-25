use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::Write;
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    execute,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use xmind_to_all::{
    json_to_sheet,
    sheet_to_tree::TestcaseTree, 
    resolve_path::AllPath, 
    unzip, 
    write_to_xlsx::write_xlsx, 
    user_config
};


enum Event {
    ProcessXmind(PathBuf),
    Exit,
}

#[tokio::main]
async fn main() {
    welcome_message();
    let(tx, mut rx) = mpsc::channel(32);
    let user_config_data = Arc::clone(&user_config::USER_CONFIG);

    // 启动事件处理任务
    let event_handler = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                Event::ProcessXmind(path) => {
                    process_xmind(path.to_str().unwrap(), Arc::clone(&user_config_data)).await;
                },
                Event::Exit => {
                    println!("按任意键退出...");
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
                Some("exit") => {
                    tx_clone.send(Event::Exit).await.unwrap();
                },
                _ => {
                    println!("Unknown command.");
                }
            }
            // 重新显示提示符
            print_prompt();
        }
    });

    // 启动光标闪烁的协程
    let blink_handle = tokio::spawn(async {
        print_prompt_with_blink().await;
    });

    // 等待任务完成
    tokio::select! {
        _ = event_handler => {},
        _ = command_handler => {},
    }

    // 处理完成后，停止光标闪烁
    blink_handle.abort(); // 停止光标闪烁协程
    print!("\x1b[?25h"); // 确保光标可见
    print_prompt(); // 打印提示符
}

fn welcome_message() {
    let help_text = r#"
欢迎使用 Xmind to All 应用程序！
Xmind to All 是一款将 Xmind 格式的测试用例转换为 XLSX 格式的工具。通过简单的命令行操作，您可以轻松转换测试用例。

命令说明：
1. process <文件名>：处理指定的 Xmind 文件并转换为 XLSX 格式。
   - 参数：文件名可以是绝对路径或放置在工作目录下的 input 目录中的文件名。
   - 示例：process test_case.xmind

2. exit：退出程序。
   - 输入 `exit` 并按回车键即可退出。

感谢您使用 Xmind to All！如需更多帮助，请联系庄啸森。
"#;

    println!("{}", help_text);
}

fn print_prompt() {
    print!("> "); // 打印提示符
    std::io::stdout().flush().unwrap(); // 刷新输出缓冲区
}

async fn print_prompt_with_blink() {
    let mut is_visible = true;
    print!("\r> "); // 打印提示符
    std::io::stdout().flush().unwrap(); // 刷新输出缓冲区

    loop {
        if is_visible {
            print!("\x1b[?25h"); // 显示光标
        } else {
            print!("\x1b[?25l"); // 隐藏光标
        }
        std::io::stdout().flush().unwrap();
        is_visible = !is_visible;
        sleep(Duration::from_millis(500)).await; // 使用 Tokio 的异步睡眠
    }
}

async fn process_xmind(xmind_file_path: &str, user_config_data: Arc<Mutex<HashMap<String, String>>>) {
    // 初始化路径结构体
    let xmind_path = PathBuf::from(xmind_file_path);
    let mut path_value = AllPath::set_allpath(xmind_path);

    // copy一份xmind为zip文件并解压，并返回content.json文件的路径
    fs::copy(path_value.xmind_path(), path_value.zip_path()).await
        .expect("复制xmind为zip时遇到无法解决的问题。");

    let mut content_path = unzip::extract_zip(path_value.zip_path())
        .unwrap_or_else(|_err| {
            panic!("zip解压时遇到无法解决的问题。");
        });
    content_path.push("content.json");
    fs::remove_file(path_value.zip_path()).await
        .expect("移除压缩包时遇到无法解决的问题。");

    AllPath::change_content_path(&mut path_value, content_path);

    // 获取content.json数据
    let content_path = path_value.content_path();
    let contents = json_to_sheet::get_sheet_json(content_path)
        .expect("获取内容时遇到无法解决的问题。");

    // 创建测试用例树
    let testcase_tree = TestcaseTree::from(&contents, &user_config_data);

    fs::copy(path_value.xlsx_tmp_path(), path_value.xlsx_path()).await
        .expect("复制xlsx模板时遇到无法解决的问题。");

    write_xlsx(testcase_tree, path_value.xlsx_path(), &user_config_data);

    // 删除解压出来的目录
    fs::remove_dir_all(path_value.zip_path().with_extension("")).await
        .expect("删除解压目录时遇到无法解决的问题。");

    // print!("{:?}", &path_value);

    println!("处理完成。");
}
