mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`")
    }

    pub fn function() {
        println!("called `my_mod::function()`")
    }

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

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod()
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

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

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:
    // プライベートな要素は、たとえパブリックなモジュール内に存在していても
    // 直接アクセスすることはできない。

    // Error! `private_function` is private
    // エラー!`private_function`はプライベート。
    my_mod::private_function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_function` is private
    // エラー！`private_function`はプライベート。
    my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_nested` is a private module
    // エラー！`private_nested`はプライベートなモジュール 。
    my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_nested` is a private module
    my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}
