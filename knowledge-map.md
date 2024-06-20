# Knowledge Map of the programming languate Rust

### 术语和常识

- function pointer - 函数指针类型

e.g. `fn() -> u32` 这是一个函数定义，一般会放在代码区，访问这个函数就需要有一个引用或者指针指向它

- 进程运行时的内存划分

堆，栈，代码区，？

- 常见的内存安全问题: 内存泄漏(非内存安全问题) , 堆栈溢出(迭代器/运行时检查), 重复释放, 悬垂指针;

- 堆内存的使用场景：1. 存放栈无法处理的内存（过大，或者长度不定，或者需要动态增长），2. 在同一个调用栈中真正需要被多个数据结构共享 3. 在多个调用栈中共享

### Error handling

- [Error handling in Rust](error_handling.md) - Rust 错误处理

其他可参考资源

- [错误处理 - Rust语言圣经(Rust Course)](https://course.rs/advance/errors.html)

- [18｜错误处理：为什么Rust的错误处理与众不同？-陈天 · Rust 编程第一课-极客时间](https://time.geekbang.org/column/article/424002)

- [18｜错误处理系统：错误的构建、传递和处理-Rust 语言从入门到实战-极客时间](https://time.geekbang.org/column/article/729009)

- [第 7 章 错误处理-Rust程序设计（第2版）-极客时间](https://time.geekbang.org/column/article/740806)
