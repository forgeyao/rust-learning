/**
 * 思考题
 * 在上面的斐波那契数列的代码中，你也许注意到计算数列中下一个数的代码在三个函数中不断重复。这不符合 DRY（Don’t Repeat Yourself）原则。你可以写一个函数把它抽取出来么？
 */

fn fib_next_num(a: &mut u64, b: &mut u64) {
    let c = *a + *b;
    *a = *b;
    *b = c;

    println!("next val is {}", b);
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        fib_next_num(&mut a, &mut b);

        i += 1;
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        fib_next_num(&mut a, &mut b);
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        fib_next_num(&mut a, &mut b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
