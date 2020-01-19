use std::convert::Infallible;
use tera::Tera;
use warp::{self, filters::BoxedFilter, path, Filter};

pub fn create_template_filter() -> BoxedFilter<(Tera,)> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*.html")).unwrap();
    warp::any().map(move || tera.clone()).boxed()
}

async fn hello(name: String, tmpl: Tera) -> Result<impl warp::Reply, Infallible> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &name);
    let payload = tmpl.render("hello.html", &ctx).unwrap();
    Ok(warp::reply::html(payload))
}

#[tokio::main]
async fn main() {
    let tmpl = create_template_filter();
    let routes = path!("hello" / String).and(tmpl).and_then(hello);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
