use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Response, Params, ResponseBuilder},
    http_component,
};
use maud::html;


#[http_component]
async fn topcloud(req: Request) -> Result<impl IntoResponse> {
    let mut router = spin_sdk::http::Router::default();
    router.get("/", hello);
    router.get("/secret", secret);
    router.get("/api", api);
    router.options("/...", process_preflight);
    Ok(router.handle(req))
}

fn hello(_req: Request, _params: Params) -> Result<Response>{
    let msg = html!(
        p {
            "Hello from Spin!"
            br;
            "Did you know..."
            br;
        }
        button hx-get="https://topcloud-vnlndzcb.fermyon.app/secret" hx-swap="outerHTML" {
            "This button has a secret?"
        }
    );
    let mut builder = ResponseBuilder::new(200);
    builder.header("Access-Control-Allow-Origin","*")
        .body(msg.into_string());
    Ok(builder.build())
}

fn secret(_req: Request, _params: Params) -> Result<Response>{
    let msg = html!(
        p {
            "You found the secret!"
        }
        button hx-get="https://topcloud-vnlndzcb.fermyon.app/api" hx-swap="outerHTML" {
            "Claim Your Reward"
        }
    );
    let mut builder = ResponseBuilder::new(200);
    builder.header("Access-Control-Allow-Origin","*")
        .body(msg.into_string());
    Ok(builder.build())
}

fn api(_req: Request, _params: Params) -> Result<Response>{
    let msg = html!(
        strong {
            p {
                "Wow! It's a..."
                br;
            }
            (42)
        }
    );
    let mut builder = ResponseBuilder::new(200);
    builder.header("Access-Control-Allow-Origin","*")
        .body(msg.into_string());
    Ok(builder.build())
}


pub(crate) fn process_preflight(_req: Request, _params: Params) -> Result<Response> {
    let mut builder = ResponseBuilder::new(200);
    builder.header("Access-Control-Allow-Origin","*")
        .header("Access-Control-Allow-Methods", "GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "*");

    Ok(builder.build())
}