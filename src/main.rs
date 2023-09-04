use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
mod find;
mod walk_tree;
use colored::*;
fn main(){
    let args: Vec<String> = env::args().collect();
    let mut t = args.len();
    let mut f = false;
    if t < 3 {
        eprintln!("使用方式:{}<目标目录><要搜索的正则表达式1><要搜索的正则表达式2>...",args[0]);
        process::exit(1);
    }
    if args[t-1].eq("-v") || args[t-1].eq("--verbose") {
        f = true;
        t = t - 1;
        //println!("所有遍历到的文件如下:");
    }
    let tl = 2;
    let mut ret: Vec<String> = Vec::new();
    for i in tl..t {
        let pattern = &args[i];
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(err) => {
                eprintln!("无效的正则表达式'{}':{}",pattern,err);
                process::exit(1);
            }
        };
        match find::find(&args[1], &regex, f, i){
            Ok(matches) => {
                if matches.is_empty(){
                    ret.clear();
                }else {
                    if i == 2 {
                        ret = matches;
                    }else {
                        let tmp: Vec<&String> = matches
                            .iter()
                            .filter(|&s| ret.contains(s))
                            .collect();
                        ret.clear();
                        for s in tmp {
                            ret.push(s.to_string());
                        }
                    }
                }
            }
            Err(error)=>{
                eprintln!("发生错误:{}",error);
                process::exit(1);
            }
        }
    }
    if ret.is_empty() {
        println!("未找到匹配项.");
    } else {
        println!("找到以下匹配项：");
        for file in ret {
            println!("{}", file.yellow());
        }
    }
}

