use mnlm::app::App;
use mnlm::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new();
    app.run().await
}