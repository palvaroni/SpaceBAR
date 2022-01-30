mod windows;

use simple_logger::SimpleLogger;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

#[derive(Copy, Clone)]
pub enum VkCodes {
    Space = 0x20,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let index = warp::path!().and(warp::fs::dir("www"));
    let trigger = warp::path!("trigger").and_then(do_keypress);
    let routes = warp::get().and(index).or(warp::post().and(trigger));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn do_keypress() -> Result<impl Reply, Rejection> {
    let status = match windows::keypress(&VkCodes::Space) {
        2 => {
            log::info!("Succesfully inputted events to stream");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        1 => {
            log::warn!("Sending either UP or DOWN failed");
            StatusCode::OK
        }
        _ => {
            log::error!("Could not push key events");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };
    Ok(status)
}
