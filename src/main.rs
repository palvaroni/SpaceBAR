mod windows;

use rust_embed::RustEmbed;
use simple_logger::SimpleLogger;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "www/"]
struct Asset;

#[derive(Copy, Clone)]
pub enum VkCodes {
    Space = 0x20,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let index = warp::path!().map(|| warp::reply::html(get_index()));
    let trigger = warp::path!("trigger").and_then(do_keypress);
    let routes = warp::get().and(index).or(warp::post().and(trigger));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn do_keypress() -> Result<impl Reply, Rejection> {
    let status = match windows::keypress(&VkCodes::Space) {
        2 => {
            log::info!("Succesfully inputted events to stream");
            StatusCode::OK
        }
        1 => {
            log::warn!("Sending either UP or DOWN failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => {
            log::error!("Could not push key events");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };
    Ok(status)
}

fn get_index() -> String {
    let file = Asset::get("index.html").unwrap();
    let text = std::str::from_utf8(file.data.as_ref()).unwrap();
    text.to_string()
}
