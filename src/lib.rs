use pgx::prelude::*;
use reqwest;
use serde::{Deserialize};

pgx::pg_module_magic!();

#[pg_extern]
fn hello_nueva_extension() -> &'static str {
    "Hello, nueva_extension"
}

#[derive(Deserialize)]
struct Cita {
    value: String,
}

#[pg_extern]
fn cita_chuck_norris() -> String {
    let response = reqwest::blocking::get("https://api.chucknorris.io/jokes/random").unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            let body: Cita = response.json().unwrap();
            return body.value;
        },
        _ => {
            return "Error".to_string();
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_nueva_extension() {
        assert_eq!("Hello, nueva_extension", crate::hello_nueva_extension());
    }

}

/// This module is required by `cargo pgx test` invocations. 
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
