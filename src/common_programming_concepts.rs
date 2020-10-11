pub mod variables {

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

    }
}