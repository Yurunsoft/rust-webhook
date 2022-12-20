use std::{convert::Infallible};
use futures::AsyncWriteExt;
use hyper::{Body, Request, Response,  Method, StatusCode};

use crate::gitee;
use crate::github;

pub struct Context {
    pub request: Request<Body>,
    pub response: Response<Body>,
    pub body: Vec<u8>,
}

pub async fn route(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut context = Context {request: request, response: Response::default(), body: Vec::new()};

    match (context.request.method(), context.request.uri().path()) {
        (&Method::GET, "/") => {
            *context.response.body_mut() = Body::from(r#"<p>Are you ok?</p>"#);
        },
        (&Method::POST, "/gitee") => {
            gitee::route_gitee(&mut context).await.unwrap();
        },
        (&Method::POST, "/github") => {
            github::route_github(&mut context).await.unwrap();
        },
        _ => {
            context.body.write(br#"<p>404! Are you ok?</p>"#).await.unwrap();
            *context.response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    context.body.write(br#"<p>Author: Yurun (<a href="https://gitee.com/Yurunsoft" target="_blank">Gitee</a> / <a href="https://github.com/Yurunsoft" target="_blank">Github</a>)</p>"#).await.unwrap();

    if !context.body.is_empty() {
        *context.response.body_mut() = Body::from(context.body);
    }

    Ok(context.response)
}
