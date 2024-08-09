# Concurrent (并发)

## Send 和 Sync

- 实现 Send 的类型可以在线程间安全的传递其所有权, 实现 Sync 的类型可以在线程间安全的共享(通过引用)
- 绝大部分类型都实现了 Send 和 Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc 等
- 可以为自定义类型实现 Send 和 Sync，但是需要 unsafe 代码块
- 可以为部分 Rust 中的类型实现 Send、Sync，但是需要使用 newtype，例如裸指针
