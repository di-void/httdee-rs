use httdee_rs::{HttDee, RequestHandlers};
use std::time::Duration;

fn main() {
    // define handlers
    let mut handlers = RequestHandlers::new();

    // GET request handlers..
    handlers.get("/", |req, mut res| {
        thread::sleep(Duration::from_secs(2));

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
}
