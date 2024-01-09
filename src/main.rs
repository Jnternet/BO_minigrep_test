use std::{process};
use BO_rs_minigrep::*;
fn main() {
    let 配置 = 输入配置::获取配置().unwrap_or_else(
        |err| {
            eprintln!("获取输入错误 :{err}");
            process::exit(1);
        }
    );
    if let Err(e) = run(&配置) {
        eprintln!("run 运行错误: {e} 文件地址:{}", 配置.文件地址);
        process::exit(3);
    }
    // println!("{:?}", 配置);
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn run_test() {
    //     let 配置 = 输入配置 { 查询字段: "to".to_string(), 文件地址: "poem.txt".to_string() , 打印全部吗: false};
    //     assert_eq!(run(&配置).unwrap(), ());
    // }
}

