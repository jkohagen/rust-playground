use log::{error, info, warn};
use log4rs;
use serde_json::Value as JsonValue;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MyPerson {
    name: String,
    age: u8,
    #[serde(rename = "isMale")]
    is_male: bool,
}

fn main() {
    hello_world();

    logging_test("message");

    json_test_01();
    json_test_02();
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

fn json_test_01() {
    let json_str = r#"{
        "name" : "Peter",
        "age": 55,
        "isMale" : false

    }"#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        info!("The name is '{}'", p["name"].as_str().unwrap());
    } else {
        warn!("Json parse error");
    }
}

fn json_test_02() {
    let json_str = r#"{
        "name" : "Peter",
        "age": 55,
        "isMale" : false

    }"#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let my_person: MyPerson = res.unwrap();
        info!("The name is '{}'", my_person.name);
        info!("Is male? -> '{}'", my_person.is_male);
    } else {
        warn!("Json parse error");
    }
}
