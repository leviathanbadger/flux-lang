use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: tower_lsp::lsp_types::InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<tower_lsp::lsp_types::InitializeResult> {
        Ok(tower_lsp::lsp_types::InitializeResult::default())
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("fluxd stub");
    let (service, socket) = LspService::new(|_client: Client| Backend);
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
