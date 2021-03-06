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
### 6. `TryFrom` / `TryInto` trait 定义，如何使用，调用  `.try_into`
[ From trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.From.html)

[ Into trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.Into.html)

[ TryFrom trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.TryFrom.html)

[ TryInto trait ](http://doc.rust-lang.org/1.58.1/std/convert/trait.TryInto.html)

会转移所有权
```rust
struct S;

struct T;

impl From<S> for T { todo!(); }

let t = T::from(s);
let t: T = s.into();


impl TryFrom<S> for T { todo!(); }

let t = T::try_from(s);
let t: T = s.try_into().unwrap();

```


### 6. `.try_into` 后面再调用 `map_err` 是为何，还有什么情况下也可以这些书写
[ map_err in Result ](http://doc.rust-lang.org/1.58.1/std/result/enum.Result.html#method.map_err)

### 7. `Borrow` trait 是干嘛用的, 什么情况下使用 调用 `.borrow()`
[ Borrow trait ](http://doc.rust-lang.org/1.58.1/std/borrow/trait.Borrow.html)

### 8. `Cow` 智能指针有何用处
[ Cow trait ](http://doc.rust-lang.org/1.58.1/std/borrow/enum.Cow.html)

### 9. `log tracing` 打印出来的时区时间问题解决 
[ 相关讨论 ](https://rustcc.cn/article?id=66e2a76e-8c65-42f7-a773-66dff1a2a21e)
```rust
// cargo.toml
tracing = "0.1.31"
tracing-subscriber = { version = "0.3.9", features = [
    "env-filter",
    "time",
    "local-time",
] }
time = { version = "0.3.7", features = ["macros"] }
```

```rust
// code
let local_time = OffsetTime::new(
    UtcOffset::from_hms(8, 0,0).unwrap(),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]")
);

tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_timer(local_time)
    .init();
```
### 10. `FromStr` trait定义，如何使用，调用 `.parse`
[ From trait ](http://doc.rust-lang.org/1.58.1/std/str/trait.FromStr.html)
```rust
struct T;

impl FromStr for T {
    todo!();
}

let s: &str = "xxxxxxx";

let t: T = s.parse().unwrap();
let t = s.parse::<T>().unwrap();
```
### 11. `&[Spec]` 如何理解
方法定义入参定义如下：
```rust
fn apply(&mut self, specs: &[Spec]) {
    
}
```

`&[Spec]` 表示 `Spec` 切片引用类型

调用上面方法的代码如下：
```rust
engine.apply(&spec.spec);
```

入参 `spec.spec` 的类型是 `Vec<Spec>`，即这边传入的是 `&Vec<Spec>` 类型的参数，为何 `apply` 方法可以识别?

在函数和方法中，Rust 提供了一个极其有用的隐式转换：`Deref` 转换。

当一个实现了 `Deref` 特征的值被传给函数或方法时，Rust会根据函数参数的要求，来决定使用该值原本的类型还是 `Deref` 后的类型；

同时，`Deref` 可以支持连续的隐式转换，直到找到适合的形式为止。

因为 `Vec` 实现了 `Deref` trait，`&Vec<T>` 会被自动解引用为 `&[t]`，符合接口定义

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