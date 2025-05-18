use tower_lsp::LspService;
use tower_lsp::Server;

#[tokio::main]
async fn main() {
    println!("fluxd stub");
    let (service, socket) = LspService::new(|_client| async move { () });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
