// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    // 标注元组类型时，直接使用()即可，而不能使用Tuple
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // iter()会返回不可变借用，即&(String, Command)，之后Rust会根据填写的(string, command)自动隐式转换有：
            // &(String, Command) => (&String, &Command)
            // 但是不能写&(string, command)，因此此时会与&(String, Command)严格匹配成功，Rust会尝试将值“move”到string中，导致报错
            // TODO: Complete the function body. You can do it!
            output.push(match *command {
                Command::Uppercase => {string.to_uppercase()},
                Command::Trim => {string.trim().to_string()},
                Command::Append(cnt) => {
                    let mut res = string.to_string();
                    if cnt > 0 {
                        let tmp = "bar".repeat(cnt);
                        res.push_str(tmp.as_str());
                    }
                    res
                }
            });
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    // into用于将自身消耗掉，并将所有权转移出去。into方法与from方法是对偶的，当为T实现了From<U>时，可以通过T::from(u)得到T，同时，可以通过U::into()得到T类型
    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
