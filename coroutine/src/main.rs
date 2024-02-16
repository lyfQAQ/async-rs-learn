use std::{thread, time::Duration};

mod future;
mod http;

use crate::http::Http;
use future::{Future, PollState};

// 手写状态机-Coroutine来模拟下面的异步代码
// async fn async_main() {
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").await;
//     println!("{txt}");
//     let txt = http::Http::get("/400/HelloAsyncAwait").await;
//     println!("{txt}");
// }

struct Coroutine {
    state: State,
}

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>), // future 1
    Wait2(Box<dyn Future<Output = String>>), // future 2
    Resolved,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    // 首次 poll 只是创建 fufure，并未执行
                    println!("Program starting");
                    let fut1 = Box::new(Http::get("/600/HelloWorld1"));
                    self.state = State::Wait1(fut1); // 转移状态
                }

                State::Wait1(ref mut fut1) => match fut1.poll() {
                    PollState::Ready(txt1) => {
                        println!("{txt1}");
                        let fut2 = Box::new(Http::get("/400/HelloWorld2"));
                        self.state = State::Wait2(fut2); // fut1 处理完，转移到开始处理 fut2的状态
                    }
                    PollState::NotReady => break PollState::NotReady, // 子 fut1 未准备好，逐层返回 NotReady
                },

                State::Wait2(ref mut fut2) => match fut2.poll() {
                    PollState::Ready(txt2) => {
                        println!("{txt2}");
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }

                    PollState::NotReady => break PollState::NotReady,
                },

                State::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                // 该 future 未执行完，模拟 excutor 执行其他任务
                println!("Schedule other tasks");
            }
            PollState::Ready(_) => break,
        }

        // Since we print every poll, slow down the loop
        thread::sleep(Duration::from_millis(100));
    }
}
