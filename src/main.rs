use log::{error, info, warn};
use log4rs;

fn main() {
    hello_world();

    logging_test("message")
}

fn hello_world() {
    println!("Hello, world!");
}

fn logging_test(msg: &str) {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("info '{}'", string_returner(msg));
    warn!("warn '{}'", string_returner(msg));
    error!("error '{}'", string_returner(msg));

    let x = 123;
    info!("x = '{}', so x is really '{}'", x, x);

    let y: i32 = 3;
    info!("x + y = '{}'", add_number(x, y));
}

fn string_returner(s: &str) -> &str {
    return s;
}

fn add_number(x: i32, y: i32) -> i32 {
    return x + y;
}
