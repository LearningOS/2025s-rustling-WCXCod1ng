// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// 编译阶段 (rustc)：
// 编译器看到 mod Foo { #[no_mangle] fn my_demo_function(...) { ... } }。
// 因为有 #[no_mangle]，编译器会为这个函数生成机器码，并将其标记为一个全局可见的、名称就是 my_demo_function 的符号。此时，它来自哪个模块已经不重要了，它的符号名被强制固定为 my_demo_function。
// 编译器看到 extern "Rust" { fn my_demo_function(...); }。这告诉编译器：“请相信我，在链接时会有一个名为 my_demo_function 的符号可用。”
// 编译器看到 #[link_name = "my_demo_function"] fn my_demo_function_alias(...)。这告诉编译器：“请相信我，在链接时 my_demo_function_alias 这个名字实际上应该使用 my_demo_function 这个符号。”
// 编译器将整个 crate 编译成一个中间文件。这个文件内部同时包含了 my_demo_function 的定义（来自 Foo 模块）和对它的引用（来自 extern 块和测试函数）。
// 链接阶段 (Linker)：
// 链接器接收到编译器生成的中间文件。
// 它扫描文件，制作一个清单。
// 已定义的符号列表：my_demo_function (地址是 0xABCDEF...)
// 需要解析的符号列表：
// 来自 test_success 对 my_demo_function 的调用，需要 my_demo_function 的地址。
// 来自 test_success 对 my_demo_function_alias 的调用，因为 #[link_name] 的存在，它也需要 my_demo_function 的地址。
// 链接器进行匹配：它看到自己需要 my_demo_function，并且在已定义的符号列表中正好有这个名字。它就把函数调用的占位符替换成 my_demo_function 的实际地址 (0xABCDEF...)。
// 所有符号都成功匹配，链接成功，最终的可执行文件生成。
// 关键点：在链接器看来，它处理的是一个“扁平”的符号列表，Foo 这个模块层级结构在编译阶段就已经被“压平”并转换为全局符号了。在链接阶段已经不存在模块的概念了

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
