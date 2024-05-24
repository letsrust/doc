# Rust 101

## Rust toolchain

可以看 [Rust lang get started](https://www.rust-lang.org/zh-CN/learn/get-started).  

- Install Rust  

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```

- rustc  

rust 编译器，用于编译 rust 代码。  

```bash
# Rust version  
rustc --version  

# Compile and run  
rustc hello_world.rs  
./hello_world  

# Compile and run in a target directory  
rustc --out-dir=target hello_world.rs  
./target/hello_world  
```

- cargo  

```bash
cargo new hello --bincargo new hello_lib --lib  

# Default is debug mode

cargo buildcargo build --release  

# Default is debug mode

cargo runcargo run --release  
cargo updatecargo update -p regex  

# Find unit test in the src directory, and find the integration test in the tests directory.

cargo test
```

Cargo 使用了缓存的方式提升构建效率，当构建时，Cargo 会将已下载的依赖包放在 CARGO_HOME 目录下，默认是 `~/.cargo`。  

**If you want to learn more, you can see the [cargo introduction](https://course.rs/cargo/intro.html).**  

- 升级 rust  

升级 rust 的版本，使用 `rustup update` 命令，如果要升级到 nightly 版本，使用 `rustup update nightly` 命令。  
如果要删除 rust，使用 `rustup self uninstall` 命令。  

## Rust Intro

学习基本语法，从下面开始入门  

- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) - 半小时快速了解 Rust，做一个初步的了解  

代码示例可以在 [plground/examples/101_half_hour_to_rust](../playground/examples/101_half_hour_to_rust.rs) 中找到。  

- [Learn rust in Y minutes](https://learnxinyminutes.com/docs/rust/)  

- [Rust Language Cheat Sheet](https://cheats.rs/)  

- [rustlings](https://github.com/rust-lang/rustlings) - https://rustlings.cool/ 以及 [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) - 通过例子学习 Rust  

- 尝试做 [Exercism.io Rust](https://exercism.org/tracks/rust) 部分的练习，和 LeetCode 类似  

- 结合 Google / Stackoverflow / [Rust Std Library](https://doc.rust-lang.org/std/) 等解决问题  

- 练习 [Advent of Code 2018 Calendar](https://adventofcode.com/)  
