pub use wca_oauth::oauth::*;

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("bruhh");
    let mut oauth = BaseOAuthBuilder::staging()
        .with_email()
        .with_secret("HK5EG0aL5co1dwtO6aBSeHqRMkbQgRq52xxG8ilqPes".to_owned(),
            "FcEvy_FI92tLWgmL5hy41x8vCc7Crfo-153m42AFNtI".to_owned(),
            LOOPBACK_URI.to_owned())
        .with_manage_competition_scope()
        .with_dob()
        .with_public()
        .authenticate_explicit(input)
        .await
        .unwrap();

    oauth.refresh().await.unwrap();

    let result = oauth
        .competitions()
        .managed_by_me()
        .page(1.try_into().unwrap())
        .send()
        .await
        .unwrap();

    println!("{:?}", result[0]);
}
