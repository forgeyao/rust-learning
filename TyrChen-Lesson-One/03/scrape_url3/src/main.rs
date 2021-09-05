/**
 * 通过 HTTP 请求 Rust 官网首页，然后把获得的 HTML 转换成 Markdown 保存起来
 *
 * 思考题
 * 在 scrape_url 的例子里，我们在代码中写死了要获取的 URL 和要输出的文件名，这太不灵活了。你能改进这个代码，从命令行参数中获取用户提供的信息来绑定 URL 和文件名么？
 */
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let mut url = "https://www.rust-lang.org/";
    let mut output = "rust.md";
    if args.len() >= 2 {
        url = &args[1];
        output = &args[2];
    }
    println!("Fetching url:{}", url);

    let body = reqwest::blocking::get(url)?.text()?;
    println!("Converting html to markdown...");

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
