use server::run;

mod server;

#[tokio::main]
async fn main() {
    run(42069).await;
}
