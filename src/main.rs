use serde_json::Value;
use wca_oauth::api_types::{Assignment, AssignmentCode};
pub use wca_oauth::oauth::*;

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("bruhh");
    let oauth = OAuthBuilder::staging()
        .with_secret(
            "MgEzgUZq9gKFrtsUODkbk0fjP_M6rhcYnUeVixql-cw".to_owned(),
            "t1BYijxyTdNsllEn4xDtpZoXXGACylFMvrv89K7y-rU".to_owned(),
            LOOPBACK_URI.to_owned(),
        )
        .with_manage_competition()
        .authenticate_explicit(input)
        .await
        .unwrap();

    let mut result = oauth.private_wcif("CubingUSANationals2023").send().await.unwrap();
    result.persons[0].assignments.push(Assignment {
        activity_id: 2999349319,
        assignment_code: AssignmentCode::Judge,
        station_number: None,
    });
    let patch = result.patch(&oauth).await;
    dbg!(&patch);
}

fn _json_eq(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => a.iter().zip(b.iter()).all(|(a, b)| _json_eq(a, b)),
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in a.iter() {
                match b.get(k) {
                    Some(b) => if !_json_eq(v, b) {
                        return false;
                    },
                    None => if !v.is_null() {
                        return false;
                    },
                }
            }
            for (k, v) in b.iter() {
                if !a.contains_key(k) && !v.is_null() {
                    return false;
                }
            }
            true
        },
        _ => false
    }
}
