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


/**
 * 整型溢出
 * 假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生整型溢出。
 * 关于这一行为 Rust 有一些有趣的规则：
 *  当在 debug 模式编译时，Rust 会检查整型溢出，若存在这些问题，则使程序在编译时 panic(崩溃,Rust 使用这个术语来表明程序因错误而退出)。
 * 
 * 在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。
 * 相反，当检测到整型溢出时，Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理。
 * 
 * 简而言之，大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值。比如在 u8 的情况下，256 变成 0，257 变成 1，依此类推。
 * 程序不会 panic，但是该变量的值可能不是你期望的值。依赖这种默认行为的代码都应该被认为是错误的代码。
 * 
 * 想要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
 * - wrapping_* 方法：该方法在所有模式都会按照补码循环溢出规则处理。例如，wrapping_add 方法会在溢出时返回从 0 开始的值。
 * - checked_* 方法：该方法仅在 debug 模式下检查溢出，若溢出则返回 None。在 release 模式下，它会按照补码循环溢出规则处理。
 * - overflowing_* 方法：该方法在所有模式下都会按照补码循环溢出规则处理，并返回溢出的布尔值。
 * - saturating_* 方法：该方法在溢出时返回该类型的最大值或最小值，取决于操作是溢出还是下溢。
 * 
 * 下面是演示一个 wrapping_* 方法的例子：
 */
fn wrapping_example() {
    let a: u8 = 255;
    let b = a.wrapping_add(1);
    println!("a: {}, b: {}", a, b);
}

/**
 * 浮点类型
 * 浮点类型数字 是带有小数点的数字，
 * 在 Rust 中浮点类型数字也有两种基本类型： f32 和 f64，分别为 32 位和 64 位大小。
 * 
 * 默认浮点类型是 f64，在现代的 CPU 中它的速度与 f32 几乎相同，但精度更高。
 * 
 * fn main() {
 *   let x = 2.0; // f64
 *   let y: f32 = 3.0; // f32
 * }
 * 
 * - 浮点数根据 IEEE-754 标准实现。f32 类型是单精度浮点型，f64 为双精度。
 */

/**
 * 浮点数陷进
 * 浮点数由于底层格式的特殊性，导致了如果在使用浮点数时不够谨慎，就可能造成危险，有两个原因：
 * 
 *  1. 浮点数往往是你想要数字的近似表达 
 * 浮点数类型是基于二进制实现的，但是我们想要计算的数字往往是基于十进制，
 * 例如 0.1 在二进制上并不存在精确的表达形式，但是在十进制上就存在。这种不匹配性导致一定的歧义性，
 * 更多的，虽然浮点数能代表真实的数值，但是由于底层格式问题，它往往受限于定长的浮点数精度，如果你想要表达完全精准的真实数字，
 * 只有使用无限精度的浮点数才行
 * 
 * 2. 浮点数在某些特性上是反直觉的 
 * 例如大家都会觉得浮点数可以进行比较，对吧？是的，它们确实可以使用 >，>= 等进行比较，但是在某些场景下，
 * 这种直觉上的比较特性反而会害了你。因为 f32 ， f64 上的比较运算实现的是 std::cmp::PartialEq 特征(类似其他语言的接口)，
 * 但是并没有实现 std::cmp::Eq 特征，但是后者在其它数值类型上都有定义
 * 
 * 为了避免上面说的两个陷进，需要遵循以下准则：
 * - 避免在浮点数上测试相等性
 * - 当结果在数学上可能存在未定义时，需要格外地小心
 * 
 */

// fn float_example() {
//     // 断言 0.1 + 0.2 等于 0.3
//     assert!(0.1 + 0.2 == 0.3);
// }

// fn float_test() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     println!("abc (f32)");
//     println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("        0.3: {:x}", (abc.2).to_bits());
//     println!();

//     println!("xyz (f64)");
//     println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();

//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

/**
 * 对于数学上未定义的结果，会产生 NaN（Not a Number）值。
 * 
 * 所有跟 NaN 交互的操作，都会返回一个 NaN 值。，而且 NaN 不能用来比较
 * 
 * 下面的代码会崩溃
 * let x = (-42.0_f32).sqrt();
 * assert_eq!(x, x);
 */
fn nan_eq() {
    // 处于防御性编程的考虑，可以使用 is_nan() 方法来检查 NaN 值
    let x = (-42.0_f32).sqrt();
    println!("x: {}", x);
    if x.is_nan() {
        println!("x is NaN!");
    }
}

/**
 * 数字运算
 * 
 * Rust 支持所有数字类型的基本数学运算：加法、减法、乘法、除法和取余。
 */
fn number_operation() {
    // 加法
    let sum = 5 + 10;
    println!("sum: {}", sum);
    // 减法
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    // 乘法
    let product = 4 * 30;
    println!("product: {}", product);
    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    // 取余
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}

/**
 * 综合性实例
 */
fn fundamental_example() {
    // 编译器会进行自动推导，给予 twenty i32 的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的形式进行类型标注
    let twenty_two = 22i32;

    // 只有同样的类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以使用 _ 进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million);

    // 定义一个 f32 数组，其中 42.0 会自动被推导为 f32 类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中的第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

/**
 * 位运算
 * 
 * Rust 上的位运算与其它语言上的一样
 */
fn bit_operation() {
    // 与运算
    let a = 0b0011u8;
    let b = 0b0101u8;
    let c = a & b;
    println!("a & b = {:04b}", c);

    // 或运算
    let c = a | b;
    println!("a | b = {:04b}", c);

    // 异或运算
    let c = a ^ b;
    println!("a ^ b = {:04b}", c);

    // 取反运算
    let c = !a;
    println!("!a = {:04b}", c);

    // 左移运算
    let c = a << 1;
    println!("a << 1 = {:04b}", c);

    // 右移运算
    let c = a >> 1;
    println!("a >> 1 = {:04b}", c);
}

/**
 * 序列（Range）
 * 
 * Rust 提供了一个非常简洁的方式，用来生成连续的数值，例如 1..5 会生成 1, 2, 3, 4。不包含 5.
 * 1..=5 会生成 1, 2, 3, 4, 5。包含 5。用途常用在循环中
 * 
 * for i in 1..5 {
 *  println!("{}", i);
 * }
 * 
 * 序列只允许用于数字或者字符类型
 * 原因：它们可以连续，同时编译器在编译过程中可以检查该序列是否为空，字符和数字值是Rust中仅有的可以用于判断是否为空的类型
 * 
 * 下面是使用字符类型的例子
 * for c in 'a'..'z' {
 *  println!("{}", c);
 * }
 */

/**
 * 使用 As 完成类型转换
 * 
 * Rust 中可以使用 As 来完成一个类型到另一个类型的转换，其中常用于将原始类型转换为其他原始类型
 * 但是它也可以完成诸如将指针类型转换为地址、地址转换为指针以及将指针转换为其他指针等功能
*/

/**
 * 有理数和无理数
 * 
 * Rust 的标准库相比其它语言，准入门槛较高，因此有理数和无理数包含在标准库中：
 * - 有理数和复数
 * - 任意大小的整数和任意精度的浮点数
 * - 固定精度的十进制小数，常用于货币相关场景
 * 
 * 好的社区已经开发出高质量的 Rust 数值库：num
 * 
 * 按照下面的步骤来引入 num 库：
 * 1. 创建新工程 cargo new complex-num && cd complex-num
 * 2. 在 Cargo.toml 中的 [dependencies] 添加 num = "0.4.0" 依赖
 * 3. 在 src/main.rs 中添加下面的代码
 * 4. 运行 cargo run
 * use num::complex::Complex;

 fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im)
 }
 */

/**
 * 字符、布尔、单元类型
 * 
 * 字符类型
 * 
 * Rust 的字符只能用 '' 来表示，而不是 ""，因为 "" 是用来表示字符串的
 * 
 * let x = '中';
 * println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
 * 
 * 布尔类型
 * ......
 * 
 * 单元类型
 * 
 * 单元类型就是 () ，唯一的值也是 ()
 */

/**
 * 语句与表达式
 * 
 * Rust 是一种基于表达式的语言，这意味着几乎所有的东西都是一个表达式
 * Rust 的函数体是由一系列语句组成的，最后一个表达式来表示返回值，例如
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}
 * 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。
 * 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，
 * 你需要能明确的区分这两个概念，但是对于很多其它语言而言，这两个往往无需区分。基于表达式是函数式语言的重要特征，表达式总要返回值。
 */

/**
 * 函数
 * 
 * 。。。
 */

fn main() {
    wrapping_example();
    // float_example();

    // float_test();

    nan_eq();

    number_operation();

    fundamental_example();

    println!("---------------------");
    bit_operation();
}
