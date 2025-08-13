// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1
    // 这里Vec<T>上并没有iter方法，它实际上是来自于[T]上的方法。这是因为Rust中存在的Deref Coercion，对于一个实现了Deref<Target=U> trait的类型T，编译器在看到&T的时候可以将其解引用为&U。对于本例则流程如下：
    // 方法查找：调用了 .iter() 方法。编译器首先在 my_fav_fruits 的原始类型 Vec<&str> 上查找名为 iter 的方法。
    // 查找失败：编译器发现 Vec<T> 类型本身并没有定义 iter 方法。
    // 启动Deref Coercion：编译器不会立即报错。它会检查 Vec<&str> 是否实现了 Deref trait。它发现 Vec<T> 实现了 Deref<Target=[T]>。
    // 自动转换：编译器自动将 &my_fav_fruits (类型为 &Vec<&str>) 转换为其 Deref 的目标类型，也就是 &[&str] (一个字符串切片的引用)。
    // 再次查找：现在，编译器在新类型 &[&str]（也就是在切片 [T] 上）上再次查找 iter 方法。
    // 查找成功！编译器在 [T] (slice) 类型上找到了 iter() 方法的实现。这个方法返回一个迭代器 Iter<T>。
    // 调用成功：方法被成功调用。

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
