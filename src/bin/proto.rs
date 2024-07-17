use std::time::Duration;
use tokio::{join, time};

trait SimpleFuture {
    // testing git auth
    type Output;

    /// This creates a state machine. If future is completed set the state to
    /// `Ready`, else call wake and set the state to `Pending`.
    fn pool(&mut self, wake: fn()) -> Pool<Self::Output>;
}

#[allow(dead_code)]
enum Pool<T> {
    Ready(T),
    Pending,
}

#[tokio::main]
async fn main() {
    async_vs_sync().await;

    let block = async {
        let a = 10;
        a
    };
    let res = block.await;
    println!("{res}");
}

fn blocking() {
    println!("this is blocking");
}

async fn async_vs_sync() {
    // async functions don't run when they are called
    let t1 = task1(10);
    let t2 = task1(5);

    // a sycnronous function, however, will run immediatialy.
    _ = blocking();
    join!(t1, t2);
}

async fn task1(n: usize) {
    for i in 1..=n {
        println!("count in \u{1b}[32mtask\u{001b}[0m: {i}");
        time::sleep(Duration::from_millis(10)).await;
    }
}
