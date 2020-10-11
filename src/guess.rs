pub mod game {

    use std::cmp::Ordering;
    use std::io;
    // Rng 是一个 trait，
// 它定义了随机数生成器应实现的方法，
// 想使用这些方法的话，
// 此 trait 必须在作用域中。
    use rand::Rng;

    pub fn guess_game() {
        println!("Guess the number!");

        loop {
            // rand::thread_rng 函数提供实际使用的随机数生成器：
            // 它位于当前执行线程的本地环境中，并从操作系统获取 seed。
            // 接下来，调用随机数生成器的 gen_range 方法。
            // 这个方法由刚才引入到作用域的 Rng trait 定义。
            let secret_number = rand::thread_rng().gen_range(1, 101);

            // println!("The secret number is: {}", secret_number);

            println!("Please input your guess.");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // 类型转换
            // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值
            // 它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量
            // 表达式中的 guess 是包含输入的原始 String 类型。
            // String 实例的 trim 方法会去除字符串开头和结尾的空白字符。
            // u32 只能由数字字符转换不过用户必须输入 enter 键才能让 read_line 返回，
            // 然而用户按下 enter 键时，会在字符串中增加一个换行（newline）符。
            // 字符串的 parse 方法 将字符串解析成数字。
            // 因为这个方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型，
            // 这里通过 let guess: u32 指定。
            let guess: u32 = match guess.trim().parse() {
                // match 语句
                // 须知 parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。
                // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。
                // 这个 Ok 值与 match 第一个分支的模式相匹配，
                // 该分支对应的动作返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量。
                // 如果 parse 不 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err。
                // Err 值不能匹配第一个 match 分支的 Ok(num) 模式，
                // 但是会匹配第二个分支的 Err(_) 模式：_ 是一个通配符值，
                // 本例中用来匹配所有 Err 值，不管其中有何种信息。
                Ok(num) => num,
                Err(_) => continue,
            };

            // 从标准库引入了一个叫做 std::cmp::Ordering 的类型。
            // 同 Result 一样， Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。
            // 这是比较两个值时可能出现的三种结果。
            // 它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。
            // 然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。
            // 使用一个 match 表达式，
            // 根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么。
            // 一个 match 表达式由 分支（arms） 构成。
            // 一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。
            // Rust 获取提供给 match 的值并挨个检查每个分支的模式。
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
            // println!("You guessed: {}", guess);
        }
    }
}