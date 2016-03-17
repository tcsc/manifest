extern crate iron;
#[macro_use(router)] extern crate router;

use iron::prelude::*;
use router::{Router};


fn handler(req: &mut Request) -> IronResult<Response> {
    let router = req.extensions.get::<Router>().unwrap();
    let name = router.find("name").unwrap_or("<anonymous>");
    let resp = Response::with((iron::status::Ok, "Hello, ".to_owned() + name));
    Ok(resp)
}

fn main() {
    let router = router!(get "/" => handler,
                         get "/:name" => handler);
    Iron::new(router).http("localhost:2345").unwrap();
}
