use std::sync::Arc;

use async_trait::async_trait;
use maelstrom::{
    Node, Result, Runtime, done,
    protocol::{Message},
};

fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "echo" {
            let echo_resp = req.body.clone().with_type("echo_ok");
            return runtime.reply(req, echo_resp).await;
        }

        done(runtime, req)
    }
}
