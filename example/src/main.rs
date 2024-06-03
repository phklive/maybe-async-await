use maybe_async_await::{maybe_async, maybe_await};

#[cfg(feature = "async")]
use tokio;

trait TestTrait {
    /// Hello
    #[maybe_async]
    fn hello(&self);

    /// World
    #[maybe_async]
    fn world(&self) -> String;
}

struct TestStruct {}

impl TestTrait for TestStruct {
    #[maybe_async]
    fn hello(&self) {
        let world = maybe_await!(self.world());
        println!("hello {}", world);
    }

    #[maybe_async]
    fn world(&self) -> String {
        "world".to_string()
    }
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let s = TestStruct {};
    println!("Ran in async.");
    s.hello().await;
}

#[cfg(not(feature = "async"))]
fn main() {
    let s = TestStruct {};
    println!("Ran in sync.");
    s.hello();
}
