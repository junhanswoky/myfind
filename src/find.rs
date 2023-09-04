use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use crate::walk_tree::walk_tree;

pub fn find<P: AsRef<Path>>(root: P, regex: &Regex,flag: bool,num: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(),regex, flag,num,&mut matches)?;
    Ok(matches)
}