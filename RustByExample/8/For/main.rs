/**
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/for.html
 */

fn main() {
    // a..b -> [a,b)
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // a..=b  -> [a,b] 两端都包含
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // iter
    {
        let names = vec!["Bob", "Frank", "Ferris"];

        // iter 是 borrow
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        println!("names: {:?}", names);
    }

    // into_iter
    {
        let names = vec!["Bob", "Frank", "Ferris"];

        // into_iter 是 move
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        // 所有权已经转移, 报错
        //println!("names: {:?}", names);
    }

    // iter_mut
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        // iter_mut 是 mut borrow
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
}
