# Hashmaps

A *hash map* allows you to associate a value with a particular key.
You may also know this by the names [*unordered map* in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[*dictionary* in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an *associative array* in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

## HashMap的基本使用
### “消费迭代器”的含义

在 Rust 中，当一个方法名以 `into_` 开头时，它通常暗示着**所有权的转移**。

"消费迭代器" 指的是一种**获取了**它所迭代的集合的**所有权**的迭代器。它不是借用集合，而是把集合整个“吃掉”（消费掉）。

当你对一个 `HashMap` 调用 `.into_keys()` 时，会发生以下事情：

1.  **`HashMap` 的所有权被转移**：原来的 `HashMap` 变量不再有效。你不能在调用 `.into_keys()` 之后再次使用它。它被 "move" 进了 `into_keys()` 方法，并转换成了一个迭代器。
2.  **迭代器返回拥有的值**：因为迭代器现在拥有了整个 `HashMap` 的数据，所以当它被遍历时，它可以交出每个元素的**所有权**。在这种情况下，它会交出每个**键（Key）的所有权**。你得到的是 `K` 类型，而不是 `&K`（对键的引用）。
3.  **不被迭代的部分被丢弃**：作为消费过程的一部分，当迭代器被销毁时（例如，`for` 循环结束），所有没有被迭代器交出的部分都会被正确地丢弃（drop）。对于 `into_keys()` 来说，这意味着所有的**值（Value）**都会被丢弃。

**比喻：拆卸一台机器来获取零件**

*   **借用迭代器 (`.keys()` 或 `.iter()`)**：就像你只是**查看**一台机器的零件清单。你看完后，机器原封不动，还在那里。你得到的是对零件的**引用（&K）**。
*   **消费迭代器 (`.into_keys()`)**：就像你把整台机器**拆开**，只为了拿出里面的引擎（键）。你现在**拥有**了这些引擎（`K`），可以把它们装到新车里。但原来的那台机器已经被拆毁，不复存在了。

---

### 如何使用 `HashMap::into_keys()`

让我们通过一个实际的例子来看看它的用法，并与非消费型迭代器进行对比。

假设我们有一个记录用户分数的 `HashMap`。

```rust
use std::collections::HashMap;

fn main() {
    // 1. 创建一个 HashMap
    // Key 是 String，Value 是 i32
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 50);
    scores.insert(String::from("Carol"), 30);

    println!("原始的 HashMap: {:?}", scores);

    // 2. 使用 .into_keys() 创建一个消费迭代器
    // scores 的所有权在这里被转移给了迭代器
    let keys_iterator = scores.into_keys();

    // 此时，变量 `scores` 已经失效了。如果你尝试使用它，编译器会报错。
    // 下面这行代码如果取消注释，将无法编译：
    // println!("尝试访问 scores: {:?}", scores);
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // error[E0382]: borrow of moved value: `scores`
    // value borrowed here after move

    println!("\n开始遍历从 .into_keys() 获得的键：");

    // 3. 遍历迭代器
    // `for` 循环会获取迭代器中每个元素的所有权。
    // 因此，这里的 `name` 的类型是 `String`，而不是 `&String`。
    for name in keys_iterator {
        // `name` 是一个拥有的 String，我们可以对它做任何事，比如修改它。
        let mut owned_name = name;
        owned_name.push_str(" (processed)");
        println!("  处理了键: {}", owned_name);
    }
}
```

**代码分析：**

1.  我们创建了一个 `HashMap<String, i32>`。
2.  `scores.into_keys()` 调用“吃掉”了 `scores`。从这一刻起，`scores` 变量就不能再被访问了。
3.  `for name in keys_iterator` 循环开始遍历。
4.  在每次迭代中，变量 `name` 接收的是一个**完整的、拥有所有权的 `String`**。我们不需要写 `&name` 或 `*name`。
5.  因为我们拥有 `name`，所以我们可以将它赋值给一个可变变量 `owned_name` 并修改它（例如使用 `push_str`）。这对于借用的 `&str` 是不可能的。
6.  当循环结束时，`HashMap` 的所有值（10, 50, 30）都已经被静默地丢弃了。

---

### 与 `.keys()` 的对比

为了加深理解，让我们看看非消费型的 `.keys()` 方法。

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 50);

    println!("使用 .keys() 进行借用遍历：");
    // .keys() 只借用了 scores，所有权没有转移
    for name in scores.keys() {
        // 这里的 `name` 的类型是 `&String` (一个引用)
        // 我们不能修改它指向的数据
        println!("  看到了键: {}", name);
        // name.push_str("!"); // <-- 这行会报错，因为 `name` 是一个不可变引用
    }

    // 遍历结束后，`scores` 仍然完全有效，可以继续使用
    println!("\n遍历后，scores 仍然可用: {:?}", scores);
}
```

### 总结：何时使用 `into_keys()`？

你应该在以下情况使用 `into_keys()`：

1.  **当你不再需要原始的 `HashMap` 时**：你只想提取出键，并且之后不会再用到那个哈希映射。
2.  **当你需要键的所有权时**：最常见的场景是，你想把这些键移动到另一个数据结构中。

**示例：将 `HashMap` 的键移动到 `Vec` 中**
```rust
let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 10);
scores.insert(String::from("Bob"), 50);

// .into_keys() 是最高效的方式，因为它直接转移所有权，无需克隆。
let names: Vec<String> = scores.into_keys().collect();

println!("现在我们有了一个只包含名字的 Vec: {:?}", names);
```

如果你在这里使用 `.keys()`，你得到的是 `Vec<&String>`（一个引用的向量），如果你需要 `Vec<String>`，你还必须额外地 `.clone()` 每个键，效率更低。
