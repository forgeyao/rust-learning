/**
 *
 *
 */

fn main() {
    // 堆引用栈数据
    {
        let mut data: Vec<&u32> = Vec::new();
        let v = 42;
        data.push(&v);
        println!("data: {:?}", data);
    }

    // 这里会报错
    {
        let mut data: Vec<&u32> = Vec::new();
        //push_local_ref(&mut data);
        println!("data: {:?}", data);
    }
}
/*
fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    data.push(&v);
}*/
