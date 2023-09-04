use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use colored::*;
pub static mut sumpath: Vec<String> = Vec::new();
pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    flag: bool,
    num: usize,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = true;
    if dir.is_dir(){
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path,regex,flag,num,matches)?;
            }else if let Some(filename) = path.file_name().and_then(|s| s.to_str()){
                if regex.is_match(filename){
                    matches.push(path.to_string_lossy().to_string());
                }
                if flag && num == 2 {
                    if f {
                        println!("所有遍历到的文件如下:");
                        f = false;
                    }
                    println!("{}", path.to_string_lossy().to_string().green());
                }
            }
        }
    }
    Ok(())
}