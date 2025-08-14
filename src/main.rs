use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

static FRAMES: [&str; 4] = [
    "[■      ][■      ][■      ]",
    "[ ■     ][ ■     ][ ■     ]",
    "[  ■    ][  ■    ][  ■    ]",
    "[   ■   ][   ■   ][   ■   ]",
];

async fn rotating_boxes_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let frames = FRAMES.iter()
        .map(|frame| format!("{}\n", frame))
        .collect::<String>();

    let response = Response::new(Body::from(format!(
        "Rotating Boxes (1 frame per request):\n{}",
        frames
    )));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(rotating_boxes_handler))
    });

    println!("Server running on http://localhost:{port}");

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
