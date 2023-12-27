use std::str;
use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Method, Response},
    http_component,
};
use serde::{Serialize, Deserialize};
use maud::html;

#[derive(Serialize, Deserialize, Debug)]
struct Fact {
    fact: String,
}

#[http_component]
async fn topcloud(_req: Request) -> Result<impl IntoResponse> {
    let req = Request::builder()
        .method(Method::Get)
        .uri("https://random-data-api.fermyon.app/animals/json")
        .build();
    let res: Response = spin_sdk::http::send(req).await?;
    let msg: Fact = serde_json::from_slice(res.body()).unwrap();
    let markup = html! {
        p {
            "Hi, did you know that..."
            br;
            (msg.fact) "?" 
        }
        button { "What do I do?" }
    };
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(markup.into_string())
        .build())
}