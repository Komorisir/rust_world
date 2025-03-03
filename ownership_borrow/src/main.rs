/**
 * 所有权
 */
// --------------------------------------------------------- 示例 1 ---------------------------------------------------------

// fn main() {
//     let s = String::from("hello"); // s 进入作用域

//     takes_ownership(s); // s 的值移动到函数里 ...
//                         // ... 所以到这里不再有效

//     // println!("在move进函数后继续使用s: {}", s); // 这里会报错

//     let x = 5; // x 进入作用域

//     makes_copy(x); // x 应该移动到函数里，
//                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
// }   // 这里, x 先移出作用域，然后是 s。但因为 s 的值已被移走，
//     // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里, some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里, some_integer 移出作用域。不会有特殊操作

// --------------------------------------------------------- 示例 2 ---------------------------------------------------------
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership 将返回值
//                                         // 移给 s1

//     let s2 = String::from("hello");     // s2 进入作用域

//     let s3 = takes_and_gives_back(s2);  // s2 被移动到
//                                         // takes_and_gives_back 中,
//                                         // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
//                                              // 调用它的函数

//     let some_string = String::from("hello"); // some_string 进入作用域.

//     some_string                              // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

//     a_string  // 返回 a_string 并移出给调用的函数
// }

/**
 * 引用与借用
 */

// --------------------------------------------------------- 示例 3 ---------------------------------------------------------
/**
 * 引用与解引用
 * 
 * 变量 x 存放了一个值 5，而变量 y 是 x 的引用
 * 
 * 可以通过断言 x 等于 5，但是想要对 y 进行断言，必须使用 *y 来解引用
 */
// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//     // assert_eq!(5, y); // 这里会报错
// }

// --------------------------------------------------------- 示例 4 ---------------------------------------------------------
/**
 * 不可变引用
 * 
 * 下面的代码，我们用 s1 的引用作为参数传递给 calculate_length 函数，而不是把 s1 的所有权转移给该函数
 * 
 * 能注意到两点：
 * - 不用像上一节那样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
 * - calculate_length 函数参数的类型是 &String 而不是 String
 */
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {  // s 是对 sString 的引用
//     s.len()
// } // 这里 s 离开作用域，但因为它并不拥有引用值的所有权，所以什么也不会发生

// --------------------------------------------------------- 示例 5 ---------------------------------------------------------
/**
 * 光是借用已经不能满足我们的需求了，如果尝试改变 s 的值，会发生什么？
 * 
 * 下面这个例子会报错，因为 s 是不可变的引用
 */
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// --------------------------------------------------------- 示例 6 ---------------------------------------------------------
/**
 * 可变引用
 * 
 * 可以创建一个可变引用，允许修改引用的值
 * 
 * - 首先声明 s 是可变类型
 * - 其次，创建一个可变的引用 &mut s 和接受可变参数 some_string: &mut String
 * - 最后，通过 some_string.push_str(", world"); 来修改引用的值
 */
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// --------------------------------------------------------- 示例 7 ---------------------------------------------------------
/**
 * ** 可变引用同时只能存在一个 **
 * 
 * 不可变引用的并不是随心所欲、想用就用的，它有一个很大的限制：在特定作用域中的特定数据只能有一个不可变引用
 * 
 * 下面的代码会报错，错误原因在于第一个可变借用 r1 必须要持续到最后一次使用的位置 println! 为止
 * 在 r1 的作用域中，不能再创建一个可变引用 r2
 * 
 * 这种限制的好处就是使 Rust 在编译期就避免了数据竞争，数据竞争可由以下的行为造成:
 * - 两个或者更多的指针同时访问同一数据
 * - 至少有一个指针被用来写入数据
 * - 没有同步数据访问的机制
 */
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// --------------------------------------------------------- 示例 8 ---------------------------------------------------------
/**
 * 可变引用与不可变引用不能同时存在
 * 
 * 下面代码会导致报错
 * 
 * 其实这个也很好理解，正在借用不可变引用的用户，肯定不希望他借用的东西，被另外一个人莫名其妙改变了。
 * 多个不可变借用被允许是因为没有人会去试图修改数据，每个人都只读这一份数据而不做修改，因此不用担心数据被污染。
 * 
 * 注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 } 结束
 */
// fn main() {
//   let mut s = String::from("hello");

//   let r1 = &s; // 没问题
//   let r2 = &s; // 没问题
//   let r3 = &mut s; // 大问题

//   println!("{}, {}, and {}", r1, r2, r3);
// }

// --------------------------------------------------------- 示例 9 ---------------------------------------------------------
/**
 * Rust 的编译器一直在优化，早期的时候，引用的作用域跟变量作用域是一致的，这对日常使用带来了很大的困扰
 * 你必须非常小心的去安排可变、不可变变量的借用，免得无法通过编译，例如以下代码：
 */
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2);
//     // 新编译器中，r1,r2作用域在这里结束

//     let r3 = &mut s;
//     println!("{}", r3);
// } // 老编译器中，r1、r2、r3作用域在这里结束
//   // 新编译器中，r3作用域在这里结束

// --------------------------------------------------------- 示例 9 ---------------------------------------------------------
/**
 * 悬垂引用
 * 
 * Rust 编译器会确保引用永远不会变成悬垂状态
 * 
 * 下面的代码会报错，因为 s 在函数结束后就被销毁了，而 r 指向的是 s 的引用
 */
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！

// // 其中一个很好的解决方法是直接返回 String
// fn no_dangle() -> String {
//   let s = String::from("hello");

//   s
// } // 最终 String 的 所有权被转移给外面的调用者
