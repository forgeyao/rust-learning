/**
 * 实现 split 函数，让下面的代码测试正确。注意不能直接使用标准库的 split 函数。
 * 提示，可以使用 str::find
 *
 * https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4de1be7767b2453fba8cb789140577a3
 */

fn main() {
    let s = String::from("hello world");
    assert_eq!(split(&s.as_str(), ' '), &["hello", "world"]);
    assert_eq!(
        split("to be, or not to be, this is a question", ','),
        &["to be", " or not to be", " this is a question"]
    );

    // 注意 pat 这里是: ❤（长度不是 1 哦）
    assert_eq!(split("我❤你❤他", '❤'), &["我", "你", "他"]);
}

fn split<'a>(s: &'a str, pat: char) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    let mut s = s;

    while let Some(i) = s.find(pat) {
        v.push(&s[..i]);
        let next = i + pat.len_utf8();
        s = &s[next..];
    }
    v.push(s);
    v
}
