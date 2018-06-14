//routing table is here



extern crate futures;
extern crate hyper;

use self::futures::future;
use self::hyper::rt::{Future};
use self::hyper::service::service_fn;
use self::hyper::{Body, Method, Request, Response, Server, StatusCode};

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try see currend address(es) /address(es)");
        }

        (&Method::GET, "/addresses") => {
//            *response.body_mut() = Body::from(get_addresses());
            *response.body_mut() = Body::from(super::sia::wallet::get_addresses_2());
        }

        (&Method::GET, "/address") => {
//            *response.body_mut() = Body::from(get_addresses());
            *response.body_mut() = Body::from(super::sia::wallet::get_first_address());
        }

        // The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

pub fn run_server() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}