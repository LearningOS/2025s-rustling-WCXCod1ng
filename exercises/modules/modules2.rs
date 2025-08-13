// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    // 使用use as可以修改模块对外部的接口，例如下面就相当于暴露了fruit作为PEAR
    // 使用self::访问当前模块下的子模块、常量或函数
    // 由于模块中定义的字段、use、fn都是私有的，需要使用pub来将其共有化（这里之所以能够访问PEAR还是因为fruits模块将其pub了出来）
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
