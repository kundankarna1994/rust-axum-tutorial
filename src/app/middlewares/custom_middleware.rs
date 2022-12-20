use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::app::handlers::home_handler::HeaderMessage;

pub async fn set_custom_header_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers.get("x-custom_header");
    match message {
        Some(x) => {
            let message = x.to_str().unwrap().to_owned();
            let extension = req.extensions_mut();
            extension.insert(HeaderMessage(message));
        }
        _ => (),
    }

    Ok(next.run(req).await)
}
