use email_newsletter_api::run_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_app().await
}
