/**
 * 查找数组中是否存在特定值
 * 存在返回下标，不存在返回 None
 */
fn main() {
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_post(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

fn find_post(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}
