use maybe_async_await::{maybe_async, maybe_await};

#[cfg(feature = "async")]
use tokio;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    hello().await;
}

#[cfg(not(feature = "async"))]
fn main() {
    hello();
}

#[maybe_async]
fn hello() {
    let world = maybe_await!(world());
    println!("hello {}", world);
}

#[maybe_async]
fn world() -> String {
    "world".to_string()
}
