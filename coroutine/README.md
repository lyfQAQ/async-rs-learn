# 手写 Coroutine 状态机
手写状态机-Coroutine来模拟下面的异步代码

```Rust
async fn async_main() {
    println!("Program starting");
    let txt = http::Http::get("/600/HelloAsyncAwait").await;
    println!("{txt}");
    let txt = http::Http::get("/400/HelloAsyncAwait").await;
    println!("{txt}");
}
```