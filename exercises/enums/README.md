# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
  当然！这是一个非常好的问题，它精准地触及了 Rust 中**模式匹配（Pattern Matching）**、**借用（Borrowing）**以及**所有权转移（Move）**之间复杂的相互作用。

## 模式匹配中的问题

简单来说，`for &(string, command)` 失败的原因是：**这个模式试图从一个借用的内容中移出（move out）其所有权，而这是 Rust 的所有权规则所禁止的。**

让我们来详细分解这两种写法的行为。

---

### 首先，理解 `input.iter()`

*   `input` 的类型是 `Vec<(String, Command)>`。
*   调用 `.iter()` 方法会创建一个迭代器。这个迭代器**不会**交出 `Vec` 中元素的所有权。相反，它会遍历并**借用** `Vec` 中的每一个元素。
*   因此，`input.iter()` 产生的每个“物品”（item）的类型是 **`&(String, Command)`**，即“对一个元组的引用”。

---

### 1. The Working Case: `for (string, command) in input.iter()`

```rust
// 迭代器产生 &(String, Command)
for (string, command) in input.iter() {
    // string 的类型是 &String
    // command 的类型是 &Command
}
```

**为什么这能行？**

这得益于 Rust 的一个强大特性，有时被称为**“匹配人体工程学”（Match Ergonomics）**或**自动解引用**。

1.  `for` 循环拿到一个类型为 `&(String, Command)` 的值。
2.  它看到左边的模式是 `(string, command)`。
3.  Rust 知道它不能将 `(String, Command)` 这个值的所有权移出，因为这个值是被 `input` `Vec` 拥有的，我们只有一个对它的引用。
4.  因此，为了让模式匹配成功，Rust 会自动地**在模式内部进行借用**。它不会尝试移动 `String` 和 `Command`，而是为它们创建引用。
5.  所以，变量 `string` 被绑定为对元组中第一个元素的引用（类型 `&String`），变量 `command` 被绑定为对第二个元素的引用（类型 `&Command`）。

这个过程是**隐式**和**自动**的，非常方便。这是 Rust 中最常见、最符合语言习惯的写法。

---

### 2. The Failing Case: `for &(string, command) in input.iter()`

```rust
// 迭代器产生 &(String, Command)
for &(string, command) in input.iter() {
    // 这里会发生编译错误！
}
```

**为什么这会失败？**

这里的模式 `&(string, command)` 是一个**显式的解引用模式**。让我们一步步分析它的含义：

1.  `for` 循环同样拿到一个类型为 `&(String, Command)` 的值。
2.  左边的模式 `&(string, command)` 对这个值进行匹配。`&` 在模式的开头意味着：“我期望一个引用，并且我要对它**内部的内容**进行匹配”。
3.  所以，模式 `(string, command)` 现在被用来匹配引用**内部**的值，这个值的类型是 `(String, Command)`。
4.  现在到了最关键的一步：默认情况下，当一个变量（如 `string` 或 `command`）在一个模式中绑定一个值时，它会尝试**移动（move）**这个值，即获取其所有权。
    *   所以，模式 `(string, command)` 试图将元组中的 `String` **移动**到新变量 `string` 中。
    *   同时，它试图将 `Command` **移动**到新变量 `command` 中。
5.  **这就是冲突所在！** 我们不能从一个共享引用 `&` 的背后移出数据的所有权。`String` 和 `Command` 都不实现 `Copy` trait，所以它们不能被简单地复制。移动它们会使 `input` `Vec` 中留下一个无效的“空洞”，这是 Rust 的所有权系统严格禁止的。

---

### 如何“修正”这个失败的模式（以及为什么你不应该这样做）

为了教学目的，你可以通过在模式内部使用 `ref` 关键字来强制进行借用，从而“修正”这个语法：

```rust
// 这可以编译通过，但不推荐
for &(ref string, ref command) in input.iter() {
    // string 的类型是 &String
    // command 的类型是 &Command
}
```

这里的 `ref` 明确地告诉编译器：“不要移动 `string`，而是创建一个对它的引用”。

但这完全是多余的，因为它只是手动实现了第一种写法中编译器自动为你做的事情。因此，这种写法非常啰嗦，也不符合 Rust 的编码习惯。

### 总结

| 写法 | `for item in input.iter()` 中 `item` 的类型 | 模式的行为 | 绑定的变量类型 | 结果 |
| :--- | :--- | :--- | :--- | :--- |
| **`for (string, command)`** | `&(String, Command)` | **隐式借用** (Match Ergonomics) | `string: &String`<br>`command: &Command` | **成功 (推荐)** |
| **`for &(string, command)`** | `&(String, Command)` | **显式解引用 + 移动** | (尝试) `string: String`<br>`command: Command` | **失败 (所有权冲突)** |
| **`for &(ref string, ref command)`** | `&(String, Command)` | **显式解引用 + 显式借用** | `string: &String`<br>`command: &Command` | **成功 (但冗余)** |

**结论：** 始终使用 `for (string, command) in input.iter()` 这种简洁、清晰且符合语言习惯的方式。它能正确地处理借用，让代码既安全又易于阅读。