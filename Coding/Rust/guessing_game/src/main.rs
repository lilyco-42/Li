// 这一行我们zed 和 rust lsp自动补全了
use std::io;

fn main() {
    println!("猜数字");
    // println! 即打印且换行
    println!("输入一个数字吧");

    // 声明一个可变长度 变量存储 用户的输入
    // ly前缀代表是我的变量
    // String 是一个字符串类型,可变长度,new是类型新建的默认方法.
    // let 声明变量
    // mut 该变量的值可以改变
    let mut ly_guess = String::new();
    // 此时可能有编译器提醒 变量未使用,黄色是警告warning
    // 忽略,后续会使用
    // 从输入输出(io)中的输入(stdin)读取一行(read_line) 存入前面的变量,若失败就提示expect()
    // &mut 表示可变借用. 借用的意思是 图书馆借书类似的概念
    io::stdin().read_line(&mut ly_guess).expect("读取失败");
    // 可以看到边写,边报错消失,我们处理了读取失败的错误情况.
    // 若无expect 返回类型是 Result 封装的数据,需要解出来
    println!("你输入的是{}", ly_guess);
    // 此时 cargo run
    // 猜数字
    //输入一个数字吧
    //98
    //你输入的是98
    let ly_rand_number = rand::random_range(0..=100);

    println!("随机数:{}", ly_rand_number);
}
