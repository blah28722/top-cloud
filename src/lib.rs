use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Method, Response},
    http_component,
};

#[http_component]
async fn topcloud(_req: Request) -> Result<impl IntoResponse> {
    // Create the outbound request object
    let req = Request::builder()
        .method(Method::Get)
        .uri("https://random-data-api.fermyon.app/animals/json")
        .build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;

    println!("{:?}", res);  // log the response
    Ok(res)
}