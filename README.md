---
# 可完善的功能
## 添加新功能：
* 首先添加新的 proto, 定义新的 spec
* 为 spec 实现 SpecTransform trait 和一些辅助函数
* 在 Engine 中使用 spec

## 更换图片引擎
* 添加新的图片引擎，像 Photon 一样，实现 Engine trait 以及为每种 spec 实现 SpecTransform Trait
* 在 main.rs 里使用新引擎

---
# 知识点
### 1. `lazy_static!` 宏里面的变量定义为何加上 `ref` 关键字，却还是表示对象，不是引用

[查看 rs.docs ](https://docs.rs/lazy_static/1.4.0/lazy_static/)

### 2. 全局变量 `static` 和 `const` 有何区别

[static keyword ](https://doc.rust-lang.org/1.58.1/std/keyword.static.html)

[const keyword ](https://doc.rust-lang.org/1.58.1/std/keyword.const.html)

### 3. `match` 中使用 `ref` 关键字，`ref` 和 `&` 有何区别

[ ref keyword ](http://doc.rust-lang.org/1.58.1/std/keyword.ref.html)

[ rust all pointer type ](http://doc.rust-lang.org/1.58.1/reference/types/pointer.html#references--and-mut)

### 4. `mod` 模块引用实践总结
[ mod keyword ](http://doc.rust-lang.org/1.58.1/std/keyword.mod.html)

[ pub keyword ](http://doc.rust-lang.org/1.58.1/std/keyword.pub.html)

[ visibility and privacy ](https://doc.rust-lang.org/1.58.1/reference/visibility-and-privacy.html?highlight=pub#visibility-and-privacy)

### 5. `From` / `Into` trait 定义，及如何使用，怎么调用  `.into`
[ From trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.From.html)

[ Into trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.Into.html)

### 6. `TryFrom` / `TryInto` trait 定义，如何使用，调用  `.try_into`
[ TryFrom trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.TryFrom.html)

[ TryInto trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.TryInto.html)

### 6. `.try_into` 后面再调用 `map_err` 是为何，还有什么情况下也可以这些书写
[ map_err in Result ](http://doc.rust-lang.org/1.58.1/std/result/enum.Result.html#method.map_err)

### 7. `Borrow` trait 是干嘛用的, 什么情况下使用 调用 `.borrow()`
[ Borrow trait ](http://doc.rust-lang.org/1.58.1/std/borrow/trait.Borrow.html)

### 8. `Cow` 智能指针有何用处
[ Cow trait ](http://doc.rust-lang.org/1.58.1/std/borrow/enum.Cow.html)

### 9. `log tracing` 打印出来的时区时间问题解决 
[ 相关讨论 ](https://rustcc.cn/article?id=66e2a76e-8c65-42f7-a773-66dff1a2a21e)

### 10. `FromStr` trait定义，如何使用，调用 `.parse`
[ From trait ](http://doc.rust-lang.org/1.58.1/std/str/trait.FromStr.html)

### 11. `&[Spec]` 如何理解

### 12. 字符串切分 split 怎么搞

### 13. `anyhow::Error` vs `std::Error` / `anyhow::Result` vs `std::Result`

### 14. `#[repr(i32)]` 这个宏干什么的？ `.from_i32`
[ repr ](http://doc.rust-lang.org/1.58.1/reference/type-layout.html#reprc-structs)
[ what's this? ](https://docs.rs/num-traits/0.2.12/num_traits/cast/trait.FromPrimitive.html#method.from_i32)

### 15. 集合 迭代器 切片 总结

### 16. 调用 `.encode_to_vec()` 为何必须要 `use prost::Message`，`#[derive(::prost::Message)]` 宏怎么实现的

### 17. 一个 `let data: Vec<u8>`, 执行 `&data[..]` 是变成什么东西

### 18. `Option` 使用， `ok_or_err` / `map` 传递闭包 

### 19. 宏编程怎么玩

### 20. 多线程 & 异步编程