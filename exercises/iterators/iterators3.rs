// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else {
        // 尝试除
        let div = a / b;
        if div * b == a {
            Ok(div)
        } else {
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: a,
                divisor: b,
            }))
        }
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    // 注意，如下的collect是支持短路计算的：如果被迭代的元素是一个Result，那么如果遇到一个Err，就返回第一个Err，否则返回Ok(collect的结果)
    numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .collect::<Vec<Result<i32, DivisionError>>>()
}

// 观察到，上述两种方式代码逻辑一模一样，区别仅在于返回值类型不同，这是因为collect会根据匹配的类型，调用对应类型的方法（要求这个类型实现了FromIterator trait）。分为如下两种情况，其实情况一是Result专门实现了FromIterator这个trait
// 情况一：目标类型 (Self): Result<Collection, Error>
// 输入迭代器 (iter): Iterator<Item = Result<T, Error>>
// 构建逻辑 (from_iter):
// 1. 创建一个空的、可变的内部集合 collection。
// 2. 开始遍历输入的迭代器。
// 3. 对于每一个元素：
// 4. 如果是 Ok(item)，则将 item 添加到内部的 collection 中。
// 5. 如果是 Err(e)，立即停止，丢弃已经收集的所有内容，并直接返回 Err(e)。这就是短路行为。
// 6. 如果迭代器遍历完成都没有遇到错误，则返回 Ok(collection)。
// 情况二：目标类型 (Self): Vec<T>
// 输入迭代器 (iter): Iterator<Item = T>
// 构建逻辑 (from_iter):
// 1. 创建一个空的 Vec<T>。
// 2. 开始遍历输入的迭代器。
// 3. 对于每一个元素 item，直接调用 vec.push(item) 将其放入 Vec 中。它根本不关心 item 本身是什么。
// 4. 当迭代器遍历完成后，返回这个 Vec。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
