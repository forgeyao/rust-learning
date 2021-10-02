/**
 *
 * https://doc.rust-lang.org/rust-by-example/mod/visibility.html
 */

mod my_mod {
    // modules 里默认是私有的
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 通过 pub 设置成公有的
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 模块内可以直接调用，即使是私有的
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // `pub(in path)`
        // pub 只在 path 里有效
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // 当前 module 里有效
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // parent module 里有效
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // crate 里有效
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // private_nested 的私有会限制这里的 pub，即使 crate 范围更大
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();

    //my_mod::nested::public_function_in_my_mod();

    //my_mod::private_function();

    //my_mod::nested::private_function();

    //my_mod::private_nested::function();

    //my_mod::private_nested::restricted_function();
}
