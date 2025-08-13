// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // 由于y中的元素是Point而非&Point，而且Point没有实现Copy trait，所以match y中的Some(p)会导致p被移动，此时y就出现了部分移动的状态
    // 使用y.as_ref()可以将Option<Point> => Option<&Point>，这时Some中匹配的p就是&Point了
    // 另一种写法是`match &y`，这时Rust的`match ergonomics`会自动处理引用，将Some中的p匹配&Point
    // 第三中写法是`match y`，但是将Some(p) => Some(ref p)，这里告诉Rust要显示匹配&Point给p，而非Point
    match y.as_ref() {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
