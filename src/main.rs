use httpdee_rs::{HttDee, RequestHandlers};

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get("/", |req, mut res| {
        println!("GET Request Object: {:?}", req);

        res.text(String::from("GET Work!"), 200);
    });

    // POST request handlers..
    handlers.post("/", |req, mut res| {
        println!("POST Request Object: {:?}", req);

        res.text(String::from("POST Work!"), 200);
    });

    // start server
    match HttDee::new(7878, handlers) {
        Ok(server) => {
            // start server
            server.start();
        }
        _ => println!("Failed to bind Socket :("),
    }

    // todo
    // have json response method implemented on response struct
}
