use std::{fs, env};

fn main() {
    // let url = "https://www.baidu.com";
    // let output = "rust.md";
    for arg in env::args() {
        println!("{}", arg);
    }

    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("Usage: cargo run <url> <output_file>")
    }
 
    let url = &args[0];
    let output = &args[1];

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
}
