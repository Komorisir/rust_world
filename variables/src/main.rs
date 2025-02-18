// fn main() {
//     /**
//      * cargo run 运行报错
//      * 原因：变量 x 是不可变的，不能修改
//      */
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }
/**
 * 选择可变还是不可变，更多的还是取决于你的使用场景
 * - 不可变 可以带来安全性但是丧失了灵活性和性能（如果你要改变，就要重新创建一个新的变量，这里还涉及到内存对象的再分配）
 * - 可变 最大的好处就是使用上的灵活性和性能上的提升
 *  例如，在使用大型数据结构或者热点代码路径（被大量频繁调用）的场景下，在同一内存位置更新实例可能比复制并返回新分配的实例要快。
 * 使用较小的数据结构时，通常创建新的实例并以更具函数式的风格来编写程序，可能更容易被理解，所以值得以较低的性能开销来确保代码清晰
 */

fn variable_attribute() {
    // 可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

/**
 * 定义了变量但是未使用，Rust 编译器会给出警告
 * 未使用的变量会导致编译器警告，可以通过在变量名前加上下划线来消除警告
 */
fn variable_not_used() {
    // 变量未使用
    let _x = 5;
    let _y = 6;
}

/**
 * 变量解构
 * let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容
 */
fn variable_deconstruction() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true 不可变, b = false 可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

/**
 * 这种使用方式跟之前的 let 保持了一致性
 * 但是 let 会重新绑定，而这里仅仅是对之前绑定的变量进行了再赋值
 */
struct Struct {
    e: i32
}

fn deconstruction_assignment() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，所以没有使用一个变量而是用 _ 来代替
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

/**
 * 变量和常量之间的差异
 * - 常量不允许使用 mut 关键字。常量不仅仅默认不可变，而且自始自终都不可变，因为常量在编译完成后，已经确定它的值。
 * - 常量使用 const 关键字而不是 let 关键字来声明，并且常量值的类型必须是明确指定的。
 * 
 * 下面是常量的例子：
 * 
 * const MAX_POINTS: u32 = 100_000;
 * 
 * - 常量名为 MAX_POINTS，类型为 u32，值为 100_000
 * - Rust 中的常量命名规范是使用全大写字母和下划线来命名常量，另外数字字面量允许使用下划线来增加可读性
 * - 常量可以在任何作用域中声明，包括全局作用域，这在 Rust 中是有效的
 */

/**
 * 变量遮蔽
 * Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的变量
 */
fn variable_shadowing() {
    let x = 5;
    // 在函数作用域内对之前的 x 进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的 x 进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn main() {
    variable_attribute();
    variable_not_used();

    variable_deconstruction();
    deconstruction_assignment();

    variable_shadowing();
}