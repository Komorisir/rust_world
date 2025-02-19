/**
 * Rust 每个值都有其确切的数据类型，总的来说可以分为两类：
 * - 基本类型（Primitive Type）：整型、浮点型、布尔型、字符型
 * - 复合类型（Compound Type）：元组、数组、引用、切片、结构体、枚举、函数、元组、数组
 * 
 * Rust 是一种静态类型语言，这意味着必须在编译时知道所有变量的类型
 * 但是这并不意味着你需要为每个变量指定类型，Rust 有能力推断出变量的类型
 * 
 * 实例：
 * - let guess = "42".parse().expect("Not a number!");
 * 
 * 编译运行会报错：无法推断类型
 * 原因：Rust 无法推断 guess 的类型，因为 parse 方法可以将字符串解析成多种数字类型
 * 
 * 修改：let guess: u32 = "42".parse().expect("Not a number!"); 或者 let guess = "42".parse::<u32>().expect("Not a number!");
 */

fn main() {
    println!("Hello, world!");
}
