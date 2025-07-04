#[tokio::main]
async fn main() {
    sable_lsp::run_server().await;
}
