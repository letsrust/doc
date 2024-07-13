use anyhow::Result;

fn main() -> Result<()> {
    closure_call_once1();
    closure_call_once2();

    closure_mut1();

    closure_fn_call();
    Ok(())
}

// 这个函数里的闭包，满足了 FnOnce trait，因为 name 被捕获后，然后返回出去，所有权发生了转移。
fn closure_call_once1() {
    let name = String::from("Alice");
    let say_something = |greeting: String| (greeting, name);

    let res = say_something("Hello".to_string());
    println!("{:?}", res);

    // Say again.
    // closure cannot be moved more than once as it is not `Copy` due to moving the variable `name` out of its environment
    // name 的所有权转移了，不能再次调用
    // let _: (String, String) = say_something("Hello".to_string());
}

fn closure_call_once2() {
    let name = String::from("Alice");

    // 闭包内部捕获的是 clone 出的数据，name 的所有权没有转移，所以这个闭包不是 FnOnce
    let say_clone = move |greeting: String| (greeting, name.clone());

    // say_clone 可以被调用多次，因为 name 被 clone 了，所有权没有转移
    println!("{:?}", say_clone("Hello".to_string()));
    println!("{:?}", say_clone("Hi".to_string()));

    // say_clone 及时不是 FnOnce, 但是一旦被当作 FnOnce 调用之后就不可以被再次调用
    let _ = call_once("aa".to_string(), say_clone);
    // let _ = call_once("arg".into(), say_clone);

    // say_clone 不能再次调用，say_clone 的所有权被转移到 call_once 里了
    // say_clone("Hello".to_string());

    println!("{:?}", call_once("haha".into(), not_closure));
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "not_closure".to_string())
}

fn closure_mut1() {
    let mut name = String::from("hello");
    let mut name1 = String::from("world");

    let mut c = || {
        name.push_str(" sss");
        println!("{}", name);
    };

    let mut c1 = move || {
        name1.push_str("!!!");
        println!("{}", name1);
    };

    c();
    c1();

    fnmut_call_mut(&mut c);
    fnmut_call_mut(&mut c1);

    fnmut_call_once(c);
    fnmut_call_once(c1);

    // c();
}

fn fnmut_call_mut(c: &mut impl FnMut()) {
    c();
}

// 下面的也可以
// fn fnmut_call_mut2<T: FnMut()>(c: &mut T) {
//     c();
// }

fn fnmut_call_once(c: impl FnOnce()) {
    c();
}

fn closure_fn_call() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    let mut c = |x: u64| v.len() as u64 * x;
    let mut c1 = move |x: u64| v1.len() as u64 * x;

    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));

    println!("fn_call: {}", fn_call(2, &c));
    println!("fn_call: {}", fn_call(2, &c1));

    println!("fn_call_mut: {}", fn_call_mut(2, &mut c));
    println!("fn_call_mut: {}", fn_call_mut(2, &mut c1));

    println!("fn_call_once: {}", fn_call_once(2, c));
    println!("fn_call_once: {}", fn_call_once(2, c1));
}

fn fn_call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn fn_call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn fn_call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}
