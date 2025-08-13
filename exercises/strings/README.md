# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)


## String vs &str
好的，这是一个 Rust 中至关重要的问题。理解 `String` 和 `&str`（字符串切片）的区别是掌握 Rust 所有权和借用系统的关键一步。

简单来说，核心区别在于**所有权**：

*   **`String`**：一个**拥有所有权**的、在堆上分配的、可增长的、UTF-8 编码的字符串。当你需要创建、修改或拥有字符串数据时，使用 `String`。
*   **`&str`**：一个**借用**的、不可变的字符串**切片**。它是一个指向某处（可能是 `String` 或字符串字面量）字符串数据的“视图”。当你只需要读取字符串数据而不需要拥有它时，使用 `&str`。

---

### 详细对比

| 特性 | `String` | `&str` (字符串切片) |
| :--- | :--- | :--- |
| **所有权** | **拥有 (Owner)** | **借用 (Borrower)** |
| **内存位置** | **堆 (Heap)**。它控制着自己的内存，并在不再需要时释放它。 | 指向任何有效的 UTF-8 序列。可以是堆上的 `String`，也可以是程序二进制文件中的静态数据。 |
| **内部结构** | 一个“胖指针”，包含三部分：<br>1. 指向堆上数据的指针 <br>2. **长度 (len)**：当前使用的字节数 <br>3. **容量 (capacity)**：在不重新分配内存的情况下可以容纳的总字节数 | 一个“胖指针”，包含两部分：<br>1. 指向数据起点的指针 <br>2. **长度 (len)**：切片的字节数 |
| **可变性** | **可变 (Mutable)**。可以增长、缩短、修改内容。 | **不可变 (Immutable)**。不能修改它所指向的内容。 |
| **创建方式** | `String::from("text")`、`"text".to_string()`、`format!` 等。 | 字符串字面量 (`"text"`)、对 `String` 取切片 (`&my_string[..]`)、函数返回的切片。 |

### 类比：原始文档 vs. 只读共享链接

*   **`String`** 就像你电脑硬盘上的一个 **Word 文档（原始文件）**。
    *   你**拥有**这个文件。
    *   你可以在任何时候**打开并修改**它（添加文字、删除段落）。
    *   当你删除这个文件时，它的内容就永远消失了。
*   **`&str`** 就像你分享给同事的一个**指向该文档的只读 Google Docs 链接**。
    *   你的同事只是在**查看**（借用）文档内容，他们并不拥有这个文件。
    *   他们**不能修改**原始文档。
    *   这个链接本身非常轻量，只包含一个地址和范围。
    *   如果原始文件被你删除了，这个链接就会失效。

---

### 方法上的不同

方法上的差异也直接反映了它们在所有权和可变性上的区别。

#### 1. 只有 `String` 拥有的方法（涉及修改和容量管理）

这些方法会改变字符串本身，所以它们必须作用于一个拥有所有权且可变的 `String`。

*   **`push_str(&str)`**: 在字符串末尾追加一个字符串切片。
    ```rust
    let mut s = String::from("foo");
    s.push_str("bar"); // s 现在是 "foobar"
    ```
*   **`push(char)`**: 在字符串末尾追加单个字符。
    ```rust
    let mut s = String::from("lo");
    s.push('l'); // s 现在是 "lol"
    ```
*   **`pop()`**: 移除并返回字符串的最后一个字符（如果有的话）。
    ```rust
    let mut s = String::from("abc");
    assert_eq!(s.pop(), Some('c'));
    ```
*   **`truncate(usize)`**: 将字符串缩短到指定的字节长度。
    ```rust
    let mut s = String::from("你好, 世界");
    s.truncate(6); // "你好" 是 6 个字节
    assert_eq!(s, "你好");
    ```
*   **`clear()`**: 清空字符串，使其长度为 0。
    ```rust
    let mut s = String::from("hello");
    s.clear();
    assert_eq!(s, "");
    ```
*   **`with_capacity(usize)`**: 创建一个具有预分配容量的空 `String`，以提高性能。

#### 2. `&str` 拥有的方法（通常用于读取、搜索和切片）

这些方法只读取数据，不进行修改。**关键点**：由于 Rust 的 **Deref Coercion** 机制，`String` 类型也可以直接调用所有这些 `&str` 的方法！这是因为当你对 `String` 调用这些方法时，Rust 会自动将 `&String` 悄悄地转换为 `&str`。

*   **`len()`**: 返回字符串的字节长度。
    ```rust
    let s = "你好"; // &'static str
    assert_eq!(s.len(), 6);
    ```
*   **`is_empty()`**: 检查字符串是否为空。
    ```rust
    assert!(! "a".is_empty());
    ```
*   **`chars()`**: 返回一个遍历所有 Unicode 字符的迭代器。
    ```rust
    for c in "🦀你好".chars() {
        println!("{}", c); // 会依次打印 🦀, 你, 好
    }
    ```
*   **`bytes()`**: 返回一个遍历所有字节的迭代器。
*   **`split(&str)`**: 根据分隔符返回一个字符串切片的迭代器。
    ```rust
    let parts: Vec<&str> = "a-b-c".split('-').collect();
    assert_eq!(parts, vec!["a", "b", "c"]);
    ```
*   **`trim()`**: 移除开头和结尾的空白字符，返回一个新的字符串切片。
    ```rust
    assert_eq!("  hello world  ".trim(), "hello world");
    ```
*   **`starts_with(&str)` / `ends_with(&str)`**: 检查字符串是否以特定模式开头或结尾。

**Deref Coercion 示例：**
```rust
let s: String = String::from("  hello world  ");

// 尽管 s 是 String 类型，但我们可以直接调用 &str 的方法
// 因为 Rust 自动将 &s (类型 &String) 转换为了 &str
let trimmed = s.trim(); // trimmed 的类型是 &str

println!("'{}'", trimmed);
```

### 总结：何时使用哪个？

*   **函数参数**：**优先使用 `&str`**。
    *   这使得你的函数更通用，因为它可以接受 `String`（通过 `&my_string`）和 `&str`（如字符串字面量）。
    *   它表示函数只是借用数据，不会夺走调用者的所有权。

    ```rust
    // 好的实践：使用 &str
    fn print_message(msg: &str) {
        println!("{}", msg);
    }

    fn main() {
        let s1 = String::from("Owned string");
        let s2 = "String literal";
        print_message(&s1); // 传递 String 的引用
        print_message(s2);  // 传递 &str
    }
    ```

*   **函数返回值**：
    *   如果你在函数内部**创建了一个新的字符串**（例如，通过 `format!` 或拼接），你**必须返回 `String`**，以将所有权转移给调用者。
    *   如果你只是返回**输入字符串的一部分**，你可以返回 `&str`（但这会涉及生命周期，是一个更高级的主题）。

*   **结构体（Struct）字段**：
    *   如果希望结构体实例**拥有**自己的字符串数据，请使用 `String`。这是最常见的情况。
    *   如果结构体只是**引用**其他地方存在的数据，并且生命周期可控，可以使用 `&str`（同样需要生命周期注解）。