// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { // 这种方式更改了函数签名，要求可以接受一个实现了Error trait的任何类型。运行时多态的三种方式：&dyn Error, &mut dyn Error, Box<dyn Error>等，本质上都需要避免直接在栈上分配该trait object，而是将其分配到堆上，栈上只留下一个fat pointer
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
