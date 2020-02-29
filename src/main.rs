#![feature(async_closure)]

// use serde_derive::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

mod render;
use render::Render;

mod layout;
use layout::Layout;

async fn render_handler(org: String, repo: String) -> Result<impl warp::Reply, warp::Rejection> {
    let render = Render::new(org, repo).await;
    let readme = render.render();
    let title = render.title();
    let layout = Layout {
        page_title: &title,
        content: &readme,
    };
    let page = layout.to_string();

    Ok(warp::reply::html(page))
}

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3030)
}

#[tokio::main]
async fn main() {
    let render = warp::path!(String / String).and_then(render_handler);

    println!("Up and running, biatch");

    let port = get_server_port();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    warp::serve(render).run(addr).await;
}
