use colored::*;

pub fn info(msg: &'static str) {
    println!("{}  {}", "[info]".cyan().bold(), &msg);
}