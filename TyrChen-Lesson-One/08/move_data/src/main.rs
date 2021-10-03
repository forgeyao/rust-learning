/**
 * 查找数组中是否存在特定值
 * 存在返回下标，不存在返回 None
 *
 * 上节例子 ../07/move_data 例子，本节改造讲借用
 */

fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    println!(
        "addr of item: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    //println!("data1: {:?}", data1); // error1
    // clone
    //println!("sum of data1: {}", sum(data1.clone()));
    //println!("data1: {:?}", data1);

    //println!("sum of data: {}", sum(data)); // error2
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
