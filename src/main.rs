use std::{convert::Infallible, error::Error};
use tera::Tera;
use warp::{self, filters::BoxedFilter, path, Filter, Reply};

pub fn create_template_filter() -> Result<BoxedFilter<(Tera,)>, tera::Error> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*.html"))?;
    Ok(warp::any().map(move || tera.clone()).boxed())
}

async fn root(tmpl: Tera) -> Result<impl Reply, Infallible> {
    let ctx = tera::Context::new();
    let payload = tmpl.render("index.html", &ctx).unwrap();
    Ok(warp::reply::html(payload))
}

async fn hello(name: String, tmpl: Tera) -> Result<impl Reply, Infallible> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &name);
    let payload = tmpl.render("hello.html", &ctx).unwrap();
    Ok(warp::reply::html(payload))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tmpl = create_template_filter()?;
    let hello_route = warp::get()
        .and(path!("hello" / String))
        .and(tmpl.clone())
        .and_then(hello);
    let root_route = warp::get()
        .and(warp::path::end())
        .and(tmpl.clone())
        .and_then(root);
    let routes = warp::any().and(hello_route.or(root_route));
    Ok(warp::serve(routes).run(([127, 0, 0, 1], 3030)).await)
}
