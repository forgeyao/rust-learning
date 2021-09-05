/**
 * 通过 HTTP 请求 Rust 官网首页，然后把获得的 HTML 转换成 Markdown 保存起来
 *
 * 第二版改动。让错误传播，可以把所有的  unwrap() 换成 ? 操作符，并让 main() 函数返回一个 Result<T, E>
 */
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url:{}", url);

    let body = reqwest::blocking::get(url)?.text()?;
    println!("Converting html to markdown...");

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
