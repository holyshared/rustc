extern crate rustc_serialize;
extern crate toml;

use toml:: { Parser, Decoder, Value };
use rustc_serialize:: { Decodable };

#[derive(RustcDecodable)]
pub struct Configuration {
    server: Server
}

#[derive(RustcDecodable)]
pub struct Server {
    host: String,
    port: i64
}

#[test]
fn test_configuration() {
    let toml = r#"
        [server]
        host = "localhost"
        port = 3000
    "#;

    let v = Parser::new(toml).parse().unwrap();
    let mut d = Decoder::new(Value::Table(v));
    let config = Configuration::decode(&mut d).unwrap();

    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.port, 3000);
}
