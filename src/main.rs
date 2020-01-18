use std::convert::Infallible;
use warp::{self, path, Filter};

async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("Hello, {}!", name))
}

#[tokio::main]
async fn main() {
    let routes = path!("hello" / String).and_then(hello);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
