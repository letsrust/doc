# Copy

标准库中对 Copy 的描述: [Copy in std::marker - Rust](https://doc.rust-lang.org/std/marker/trait.Copy.html)

- 首先，Copy 是一个 marker trait

- 其次，实现了 Copy 语义的类型，在使用时会发生按位 Copy，不会发生所有权的转移，这是跟 Move 不同的地方
  
  - Move 同样也会触发按位 copy 数据给新的变量，但是会发生所有权的转移，这是默认行为

- Copy trait 的父 trait 是 Clone，那他们俩的区别是什么
  
  - 实现了 Copy trait，在赋值时 (比如 y = x)，这个 copy 的行为是隐式完成的
  
  - 实现了 Clone，没有实现 Copy 的话，需要显式调用 x.clone()

什么时候自定义类型是可以 Copy 的？

- A type can implement `Copy` if all of its components implement `Copy`.

Generally speaking, if your type *can* implement `Copy`, it should. Keep in mind, though, that implementing `Copy` is part of the public API of your type. If the type might become non-`Copy` in the future, it could be prudent to omit the `Copy` implementation now, to avoid a breaking API change.

可变引用为什么没有实现 Copy ?

比如 &mut T，是一个可变引用，因为它指向的堆内存的区域可能会发生变化，比如一个 Vec，push 数据后可能会触发扩容，导致指向的内存地址发生变化，原来的地址就会被释放，如果可以实现 Copy，那么新 Copy 出的指针就会成为悬空指针
