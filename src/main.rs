pub use wca_oauth::oauth::*;

#[tokio::main]
async fn main() {
        let mut input = String::new();
        std::io::stdin()
                .read_line(&mut input)
                .expect("bruhh");
        let oauth = OAuthBuilder::staging()
                .with_email()
                .with_secret(
                        "hMlA3cNgscCaH47y4G6qj8ZTjt99BdkBS7vqmT733pA".to_owned(),
                        "jd6p-RvVUTKA6WqtwOc2fgWhFCDIP7YtE9rBMkwoKWw".to_owned(),
                        LOOPBACK_URI.to_owned(),
                )
                .with_manage_competition()
                .with_dob()
                .with_public()
                .authenticate_explicit(input)
                .await
                .unwrap();

        let result = oauth.me().send().await.unwrap();

        println!("{:?}", result);
}
