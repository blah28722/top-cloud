use spin_sdk::http::{Request, Response, IntoResponse};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn top_cloud(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}