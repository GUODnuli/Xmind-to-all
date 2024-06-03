use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 获取输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    let release_dir = PathBuf::from(out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf();

    // 定义要创建的目录
    let input_dir = release_dir.join("input");
    let output_dir = release_dir.join("output");
    let template_dir = release_dir.join("template");

    // 创建目录
    fs::create_dir_all(&input_dir).expect("Failed to create input directory");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    fs::create_dir_all(&template_dir).expect("Failed to create template directory");

    // 定义要复制的文件
    let files_to_copy = vec![
        ("input/tmp.xmind", input_dir.join("tmp.xmind")),
        ("template/template.xlsx", template_dir.join("template.xlsx")),
    ];

    // 复制文件并打印调试信息
    for (src, dest) in files_to_copy {
        println!("Copying from {:?} to {:?}", src, dest);
        fs::copy(src, dest).expect("Failed to copy file");
    }
}