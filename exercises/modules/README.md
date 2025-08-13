# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)


## Package、Crate和Module
### 1. Package (包)

**Package 是 Cargo 的一个功能，是您能构建、测试和分享的项目的最高层级。**

-   **物理表现**: 一个包含 `Cargo.toml` 文件的项目目录。当您运行 `cargo new my_project` 时，您创建的就是一个 Package。
-   **作用**:
    1.  **定义元数据**: `Cargo.toml` 文件描述了项目名称、版本、作者等信息。
    2.  **管理依赖**: 它声明了您的项目需要依赖哪些外部的 Crate。
    3.  **组织 Crate**: 一个 Package 包含一个或多个 Crate。

-   **核心规则**: 一个 Package **最多只能包含一个库 Crate (library crate)**，但可以包含**任意多个二进制 Crate (binary crate)**。

---

### 2. Crate (单元包)

**Crate 是 Rust 的一个编译单元。它是 Rust 编译器 (`rustc`) 一次性处理的最小代码单位。** 当编译器运行时，它操作的对象就是一个 Crate。

Crate 有两种类型：

#### a) Library Crate (库单元包)

-   **目的**: 生成一个可以被其他程序或库使用的“库”文件 (`.rlib` 或 `.so`, `.dll` 等)。它提供可复用的功能。
-   **入口**: 按照约定，库 Crate 的根文件是 `src/lib.rs`。
-   **特点**: 它没有 `main` 函数。

#### b) Binary Crate (二进制单元包)

-   **目的**: 生成一个可以直接运行的可执行文件（如 `.exe` 文件或 Unix 可执行程序）。
-   **入口**:
    *   默认的二进制 Crate 根文件是 `src/main.rs`。
    *   您也可以在 `src/bin/` 目录下放置多个文件，每个文件都会被编译成一个独立的可执行文件（即一个独立的二进制 Crate）。
-   **特点**: 它必须有一个 `main` 函数作为程序入口。

一个 Package 可以同时拥有一个 `src/lib.rs` 和一个 `src/main.rs`。在这种情况下，这个 Package 就包含两个 Crate：一个库 Crate 和一个二进制 Crate。二进制 Crate 通常会依赖并使用同一个 Package 内的库 Crate 的功能。

---

### 3. Module (模块)

**Module 是在单个 Crate 内部组织代码、控制可见性（公有/私有）和划分命名空间的方式。**

-   **目的**:
    1.  **代码组织**: 将相关的代码（如函数、结构体、枚举等）组织在一起，使代码结构更清晰、更易于维护。
    2.  **控制可见性 (Privacy)**: 默认情况下，模块中的所有内容都是私有的。您必须使用 `pub` 关键字来使其对外部可见。
    3.  **防止命名冲突**: 模块创建了独立的命名空间。

-   **模块树 (Module Tree)**: 每个 Crate 都有一个模块树。树的根部是一个匿名的模块，被称为 **Crate Root**（就是 `src/lib.rs` 或 `src/main.rs` 文件本身）。

-   **声明方式**:
    1.  **行内模块**:
        ```rust
        mod front_of_house {
            // ... 内容 ...
            pub mod hosting {
                // ... 内容 ...
            }
        }
        ```
    2.  **在单独的文件中**:
        在 `src/lib.rs` 中：
        ```rust
        mod front_of_house; // 告诉编译器去加载 front_of_house.rs 或 front_of_house/mod.rs
        ```        然后创建一个 `src/front_of_house.rs` 文件，其内容就是该模块的内容。

-   **路径 (Path)**: 您可以使用路径来访问模块中的项，例如 `crate::front_of_house::hosting::add_to_waitlist();`。

---

### 总结与区别

| 特性 | Package (包) | Crate (单元包) | Module (模块) |
| :--- | :--- | :--- | :--- |
| **概念层级** | **最高层级**：项目管理和构建 | **中间层级**：编译单元 | **最低层级**：代码组织 |
| **作用** | 管理依赖、定义元数据、组织 Crate | 生成库或可执行文件 | 划分命名空间、控制可见性 |
| **物理表现** | 一个包含 `Cargo.toml` 的目录 | `src/lib.rs` 或 `src/main.rs` (Crate Root) | `mod` 关键字定义的代码块或单独的 `.rs` 文件 |
| **核心工具** | 由 **Cargo** 管理 | 由 **rustc (编译器)** 处理 | 由 **`mod` 和 `pub` 关键字** 定义 |
| **比喻** | 一个完整的**项目工程** | 一个**程序**或一个**库** | 程序中的一个**章节**或**文件** |

### 示例：将所有概念放在一起

假设我们创建了一个名为 `my_restaurant` 的项目：

```
$ cargo new my_restaurant
```

这会生成如下结构：

```
my_restaurant/
├── Cargo.toml
└── src/
    └── main.rs
```

1.  **Package**: 整个 `my_restaurant` 文件夹就是一个 Package。`Cargo.toml` 定义了它的属性。
2.  **Crate**: `src/main.rs` 是一个**二进制 Crate**的根。目前，这个 Package 只包含这一个 Crate。

现在，我们想把餐厅的业务逻辑提取成一个库，并让主程序调用它。

我们创建 `src/lib.rs`：

**`src/lib.rs`**
```rust
// Crate Root of the library crate

// 定义一个名为 'front_of_house' 的模块
pub mod front_of_house {
    // 再定义一个子模块
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 使用绝对路径调用模块中的函数
    crate::front_of_house::hosting::add_to_waitlist();
    println!("Time to eat!");
}
```

然后修改 `src/main.rs` 来使用这个库：

**`src/main.rs`**
```rust
// Crate Root of the binary crate

// 使用 `use` 关键字来引用同一个 Package 下的库 Crate
// `my_restaurant` 是 Package 的名字，Cargo 会自动处理链接
use my_restaurant::eat_at_restaurant; 

fn main() {
    eat_at_restaurant();
}
```

在这个最终的结构中：
-   **Package**: `my_restaurant`。
-   **Crates**:
    1.  一个**库 Crate**，其根是 `src/lib.rs`。
    2.  一个**二进制 Crate**，其根是 `src/main.rs`。
-   **Modules**:
    -   在库 Crate 中，我们定义了 `front_of_house` 和 `hosting` 两个模块来组织代码。
    -   `eat_at_restaurant` 函数被定义在库 Crate 的根模块中。

通过这个例子，您可以清晰地看到这三个概念是如何协同工作，构建出一个结构良好、易于维护的 Rust 项目的。