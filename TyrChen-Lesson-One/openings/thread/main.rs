fn main() {
    let name = "Tyr".to_string();
    // 原文代码是用来说明线程问题以及编译器友好提示
    /*std::thread::spawn(|| {
        println!("hello {}", name);
    });*/
    // 下面是修改正确的
    std::thread::spawn(move || {
        println!("hello {}", name);
    });

    // 这段是特意加的，为了看到输出
    // 主线程直接结束，导致 spawn 出来的线程还没来得及打印程序就退出了
    std::thread::sleep(std::time::Duration::from_nanos(1));
}
