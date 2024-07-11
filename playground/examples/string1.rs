fn main() {
    // 类型 &str, 这是字符串字面量的表示形式，
    // 1. 一般被硬编码到程序代码中，维护在内存的只读数据段中，具有全局生命周期
    // 2. 此外，字符串字面量本身是不可变的
    let s = "hello";
    println!("{}", s);
    // let mut s1 = "world";
    // s1 = s;

    // String 是分配在 heap 上的，所以内存是可以动态变化的
    let mut ss = String::from("welcome");
    ss.push_str(" to the Rust world");
    println!("{}", ss);

    // 切片
    let ss0 = &ss[0..7];
    println!("slice - {}", ss0);
    let _ss1 = &ss[..7];
    let ss2 = &ss[7..];
    println!("slice - {}", ss2);
}