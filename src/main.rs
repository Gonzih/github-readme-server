#![feature(async_closure)]

// use serde_derive::{Deserialize, Serialize};
use warp::Filter;

mod render;
use render::Render;

mod layout;
use layout::Layout;

async fn render_handler(org: String, repo: String) -> Result<impl warp::Reply, warp::Rejection> {
    let render = Render::new(org, repo);
    let readme = render.render().await;
    let layout = Layout {
        page_title: "README",
        content: &readme,
    };
    let page = layout.to_string();

    Ok(warp::reply::html(page))
}

#[tokio::main]
async fn main() {
    let render = warp::path!(String / String).and_then(render_handler);

    println!("Up and running, biatch");

    warp::serve(render).run(([127, 0, 0, 1], 3030)).await;
}
