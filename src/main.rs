use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::process;
use regex::Regex;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件路径参数
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // 读取文件内容
    let mut file_content = String::new();
    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut file_content) {
                eprintln!("Error reading file: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    }

    // 创建一个正则表达式来匹配 https 或 http 链接
    let url_regex = Regex::new(r"https?://[^\s]+").unwrap();

    // 处理文件内容，按行处理并去除非链接文本
    let modified_content: String = file_content
        .lines()
        .filter_map(|line| {
            // 使用正则表达式提取所有符合条件的链接
            let links: Vec<&str> = url_regex.find_iter(line).map(|m| m.as_str()).collect();

            // 如果有链接，则替换 `https` 为 `go run main.go https` 并返回该行
            if !links.is_empty() {
                let modified_line = links
                    .iter()
                    .map(|&link| link.replace("https", "go run main.go https"))
                    .collect::<Vec<String>>()
                    .join(" "); // 合并多个链接，使用空格分隔
                Some(modified_line)
            } else {
                // 如果没有链接，返回 None (删除该行)
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" && "); // 将所有处理过的行重新连接成文件内容

    // 在文件内容前加一行 "cd p"
    // let final_content = format!("cd p\n{}", modified_content);

    // 修改文件后缀名为 .cmd
    // let path = Path::new(file_path);
    // let new_file_path = path.with_extension("cmd");

    
    // 将修改后的内容写入新的文件
    match fs::write(&file_path, modified_content) {
        Ok(_) => println!("File content updated."),
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            process::exit(1);
        }
    }

    // let _ = to_crlf(new_file_path);

    // match fs::remove_file(&file_path) {
    //     Ok(_) => println!("Original File deleted: {}", &file_path),
    //     Err(e) => eprintln!("Error deleting original file: {}", e),
    // };
    
}
// fn to_crlf(file_path: PathBuf) -> io::Result<()> {
//     // 读取文件内容
//     let content = fs::read_to_string(&file_path)?;
    
//     // 修改换行符为 CR-LF
//     let updated_content = content.replace("\n", "\r\n").replace("\r\r\n", "\r\n");

//     // 写回文件
//     fs::write(&file_path, updated_content)?;
    
//     Ok(())
// }