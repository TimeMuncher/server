extern crate rustc_serialize;
#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel, HttpRouter, JsonBody};
// use hyper::method::Method;

#[derive(RustcDecodable, RustcEncodable)]
struct Project {
    name: String
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(explicit_router());
    // server.mount("/assets/", StaticFilesHandler::new("assets/"));

    server.listen("127.0.0.1:6767");
}

fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    router.get("/", middleware! { |_res|
        "Welcome!"
    });

    router.post("/new_project_name", middleware! { |request, response|
        let project = request.json_as::<Project>().unwrap();
        println!("New Project: {}", project.name);
    });

    // router.get("/:name/:format", middleware! { |request|
    //     let name = request.param("name").unwrap();
    //     let format = request.param("format").unwrap();
    //     format!("Name is '{}'. The requested format is '{}'.", name, format)
    // });

    router
}
