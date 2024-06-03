use maybe_async_await::{maybe_async, maybe_await};

trait TestTrait {
    #[maybe_async]
    fn hello(&self);

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
use tokio;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let s = TestStruct {};
    maybe_await!(s.hello());
}

#[cfg(not(feature = "async"))]
fn main() {
    let s = TestStruct {};
    s.hello();
}
