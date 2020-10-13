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
        const  Y: i32 = 100;
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
        let x = x+2;
        let x = x*2;
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
    pub fn scalar(){
        // 允许使用 _ 做为分隔符以方便读数，例如1_000。
        let num: u32 = 98_222;
        println!("num is {}", num);

        let char = b'A';
        let char2 = 'a';
        println!("char A is {}, char2 a is {}", char, char2);
    }
}