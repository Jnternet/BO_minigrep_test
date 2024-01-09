use std::{env, fs, process};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct 输入配置 {
    pub 查询字段: Option<String>,
    pub 文件地址: String,
    打印全部吗: bool,
}

impl 输入配置 {
    // pub fn 获取配置() -> Result<输入配置, String> {
    //     let 命令行参数: Vec<String> = env::args().skip(1).collect();
    //     if 命令行参数.len() < 2 { return Err(format!("命令行参数过少 :{:?}", 命令行参数)); }
    //     if 命令行参数.len() > 2 { return Err(format!("命令行参数过多 :{:?}", 命令行参数)); }
    //     if 命令行参数[1] == "-all" { return Ok(输入配置 { 查询字段: None, 文件地址: 命令行参数[0].clone(), 打印全部吗: true }); }
    //     let 查询字段 = 命令行参数[0].clone();
    //     let 文件地址 = 命令行参数[1].clone();
    //     Ok(输入配置 { 查询字段: Some(查询字段), 文件地址, 打印全部吗: false })
    // }
    pub fn 获取配置() -> Result<输入配置, &'static str> {
        let mut 命令行输入 = env::args();
        命令行输入.next();
        let 查询字段 = match 命令行输入.next() {
            None => { return Err("未能获取查询字段"); }
            Some(s) => { s }
        };
        let 文件地址 = match 命令行输入.next() {
            None => { return Err("未能获取文件地址"); }
            Some(s) => { s }
        };
        return if 文件地址 == "-all" { Ok(输入配置 { 查询字段: None, 文件地址: 查询字段, 打印全部吗: true }) } else { Ok(输入配置 { 查询字段: Some(查询字段), 文件地址, 打印全部吗: false }) }
    }
}

fn 打印全部(内容: &String) {
    for line in 内容.lines() {
        if !line.is_empty() { println!("{line}"); }
    }
}

pub fn run(配置: &输入配置) -> Result<(), Box<dyn Error>> {
    let 内容 = fs::read_to_string(&配置.文件地址)?;
    if 配置.打印全部吗 {
        打印全部(&内容);
        return Ok(());
    }
    let 结果 = 查找(&配置.查询字段, &内容).unwrap_or_else(
        || {
            eprintln!("未找到该字段");
            process::exit(2);
        }
    );
    for 行 in 结果 {
        println!("{}", 行);
    }
    Ok(())
}


fn 查找<'a>(字段: &Option<String>, 内容: &'a String) -> Option<Vec<&'a str>> {
    // let mut v = Vec::new();
    // for 行 in 内容.lines() {
    //     if 行.contains(&字段.clone().unwrap()) { v.push(行); }
    // }
    // if v.is_empty() { return None; }
    // Some(v)
    let 字段 = 字段.clone().unwrap();
    Some(内容.lines()
        .filter(|行| 行.contains(&字段))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn 获取配置_test() {
    //     let 配置 = 获取配置();
    //     assert_eq!(配置, Ok(输入配置 { 查询字段: "to".to_string(), 文件地址: "poem.txt".to_string() }));
    // }
}