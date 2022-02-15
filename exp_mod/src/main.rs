// mod exp_dir;
//
// fn main() {
//     crate::exp_dir::dir_test::fn_sky_test();
//     crate::exp_dir::dir::sky_dir();
//     crate::exp_dir::dir::sky_dir1();
//     println!("Hello, world!");
// }

//第一层 mod，要是mod能能引用，所有的pub的函数与属性都可以使用，因为mod的包是所有属性、方法、子模块的集合
mod outer {
    pub(super) fn outer_foo() {
        inner::inner_foo();
    }

    // 要是第一层能引用 super在外面inner也是可以使用的
    pub(super) mod inner {
        pub(super) fn inner_foo() {
            println!("hello world!");
        }
        pub mod inner1 {
            pub fn inner_foo1() {
                println!("hello world!");
            }
        }
    }
}

fn top_level_foo() {
    outer::outer_foo();
}

fn main() {
    top_level_foo();
    outer::inner::inner1::inner_foo1()
}
