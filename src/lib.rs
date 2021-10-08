use pgx::*;
use rand::seq::SliceRandom;
use serde::*;
use serde_json::Value;

extern crate jq_rs;

pg_module_magic!();

#[pg_extern]
fn what_is_something_carlson_likes() -> &'static str {
    let stuff = vec![
        "bananas",
        "citrus beers",
        "macOS",
        "Xenix",
        "FreeBSD",
        "Ivermectin",
        "Hydrogen Peroxide",
        "Sunflower Seed Butter",
        "arguing online",
        "his famous cousin",
        "Ruby",
        "Terraform",
        "Finicky APIs",
        "Car rider pickup line at school",
    ];
    return stuff.choose(&mut rand::thread_rng()).unwrap();
}

fn _json_x_jq<T: Serialize>(jq_code: &str, blob: T) -> Value {
    let raw_blob = serde_json::to_string(&blob).unwrap();
    let _prog = jq_rs::compile(jq_code);
    let _prog = match _prog {
        Ok(prog) => prog,
        Err(error) => {
            println!("_prog error: {}", error.to_string());
            panic!("jsonX_jq[compile]: {error}", error = error);
        }
    };
    let run_result = jq_rs::run(jq_code, &raw_blob);
    let run_result = match run_result {
        Ok(data) => data,
        Err(error) => panic!("jsonX_jq[jq_run]: {error}", error = error),
    };

    let json_blob = serde_json::from_str(&run_result);
    let json_blob = match json_blob {
        Ok(value) => value,
        Err(_error) => {
            panic!("Result was not valid json, jq query must yield json")
        }
    };
    json_blob
}

#[pg_extern]
fn jsonb_jq(jq_code: &str, blob: JsonB) -> JsonB {
    pgx::JsonB(_json_x_jq(jq_code, blob))
}

#[pg_extern]
fn json_jq(jq_code: &str, blob: Json) -> Json {
    pgx::Json(_json_x_jq(jq_code, blob))
}

// TODO(chad): write some actual tests, no idea how though.
#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pg_jq() {
        assert_eq!("Hello, pg_jq", crate::hello_pg_jq());
    }
}

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
