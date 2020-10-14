pub mod variables {
    pub const HEARTBEAT_127: i8 = 127;

    // 变量默认是不可改变的（immutable）
    pub fn var() {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
        // Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性
        // 在声明它的作用域之中，常量在整个程序生命周期中都有效，这使得常量可以作为多处代码使用的全局范围的值
        const Y: i32 = 100;
        println!("const value of y is : {}", Y);

        println!("heartbeat is {}", HEARTBEAT_127);
    }

    pub fn shadowing() {
        // 我们可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量。
        // Rustacean 们称之为第一个变量被第二个 隐藏 了，这意味着使用这个变量时会看到第二个值。
        // 可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏
        // 与 mut 的区别：
        // 通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不变的。
        // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字。
        let x = 2;
        let x = x + 2;
        let x = x * 2;
        println!("shadowing x is {}", x);

        let space = "     ";
        let space = space.len();

        println!("space len is {}", space);
    }
}

pub mod data_types {
    #![allow(unused)]

    pub fn statically_typed() {
        let guess: u32 = "42".parse().expect("Not a number!");
        println!("guess is {}", guess);

        let guess: f32 = "4.2".parse().expect("Not a number!");
        println!("guess is {}", guess);
    }

    // 标量
    // 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
    pub fn scalar() {
        // 允许使用 _ 做为分隔符以方便读数，例如1_000。

        // 整数字面值
        // Decimal (十进制)	98_222
        // Hex (十六进制)	0xff
        // Octal (八进制)	0o77
        // Binary (二进制)	0b1111_0000
        // Byte (单字节字符)(仅限于u8)	b'A'

        let num: u32 = 98_222;
        println!("num is {}", num);

        let char = b'A';
        let char2 = 'a';
        println!("char A is {}, char2 a is {}", char, char2);

        let f1 = 3.0;
        let f2: f32 = 6.3;
        println!("float f1 is {}, f2 a is {}", f1, f2);

        // 加法
        let sum = 5 + 10;
        println!("sum is {}", sum);

        // 减法
        let difference = 95.5 - 4.3;
        println!("difference is {}", difference);

        // 乘法
        let product = 4 * 30;
        println!("product is {}", product);

        // 除法
        let quotient = 56.7 / 32.2;
        println!("quotient is {}", quotient);

        // 取余
        let remainder = 43 % 5;
        println!("remainder is {}", remainder);

        let t = true;
        println!("bool is {}", t);

        let f: bool = false; // 显式指定类型注解
        println!("bool f is {}", f);

        // 字符类型
        // Rust 的 char 类型的大小为四个字节(four bytes)，
        // 并代表了一个 Unicode 标量值（Unicode Scalar Value），
        // 这意味着它可以比 ASCII 表示更多内容。
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("char c is {}, z is {}, heart_eyed_cat is {}", c, z, heart_eyed_cat);
    }

    // 复合类型
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
    pub fn compound_types() {
        // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
        // 元组长度固定：一旦声明，其长度不会增大或缩小。
        let tup: (i32, char, f64) = (50, '🐈', 90.0);

        // tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。
        // 为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值，
        let (x, y, z) = tup; // 解构（destructuring
        println!("tuple x is {}, y is {}, z is {}", x, y, z);

        // 使用点号（.）后跟值的索引来直接访问它们。
        // 元组的第一个索引值是 0。
        println!("tuple tup 0 is {}, tup 1 is {}, tup 2 is {}", tup.0, tup.1, tup.2);


        // array
        // 与元组不同，数组中的每个元素的类型必须相同。
        //  Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
        // rust 的数组也是在栈上的
        let a = [1, 2, 3, 4, 5];

        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];


        // 可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
        let atype: [u128; 5] = [1, 2, 3, 4, 5];

        // 初始化数组的另一种语法：
        //  如果要为每个元素创建包含相同值的数组，
        //  可以指定初始值，后跟分号，然后在方括号中指定数组的长度
        let btype = [3; 5];  // 与 let btype = [3,3,3,3,3]; 等效
    }
}


// 函数
// rust的函数和变量名都是 snake case 规范风格
// 在 snake case 中，所有字母都是小写并使用下划线分隔单词。
pub mod functions_work {
    pub  fn this_is_function(){
        println!("this is function");
    }
    pub fn func_parameters(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

    // 语句和表达式
    // 函数体由一系列的语句和一个可选的结尾表达式构成。
    // Rust 是一门基于表达式（expression-based）的语言
    // 语句（Statements）是执行一些操作但不返回值的指令。
    // 表达式（Expressions）计算并产生一个值。

    // 函数定义也是语句
    // 语句不返回值。
    // 因此，不能把 let 语句赋值给另一个变量
    pub fn statements_expressions(){
        let y = 6;     // 一个语句
        // let z = (let zz = 8);

        // 表达式会计算出一些值
        // 考虑一个简单的数学运算，比如 5 + 6，这是一个表达式并计算出值 11。
        // 表达式可以是语句的一部分：在示例 3-1 中，语句 let y = 6; 中的 6 是一个表达式，它计算出的值是 6。
        // 我们用来创建新作用域的大括号（代码块），{}，也是一个表达式
        // 函数调用是一个表达式。宏调用是一个表达式。
        let aa = {
            // 注意结尾没有分号的那一行 x+1，与你见过的大部分代码行不同。
            // 表达式的结尾没有分号。
            // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
            let y = 10;
            y+9
        };
        println!("expression aa {}", aa);
        println!("expression y {}", y)

    }

    // 函数可以向调用它的代码返回值。
    // 我们并不对返回值命名，但要在箭头（->）后声明它的类型。
    // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
    pub fn return_value() -> i32 {
        5
    }
}