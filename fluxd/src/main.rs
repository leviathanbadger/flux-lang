use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    Position, Range,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use flux_lang::parse_program;
use flux_lang::syntax::ParseError;

#[derive(Clone)]
struct Backend {
    client: Client,
}

fn syntax_diagnostics(text: &str) -> Vec<Diagnostic> {
    match parse_program(text) {
        Ok(_) => Vec::new(),
        Err(err) => {
            if let Some(parse_err) = err.downcast_ref::<ParseError>() {
                let position = Position::new(parse_err.line as u32, parse_err.column as u32);
                vec![Diagnostic {
                    range: Range::new(position, position),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: parse_err.message.clone(),
                    ..Default::default()
                }]
            } else {
                vec![Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    ..Default::default()
                }]
            }
        }
    }
}

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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let diagnostics = syntax_diagnostics(&text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.last() {
            let diagnostics = syntax_diagnostics(&change.text);
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    println!("fluxd stub");
    let (service, socket) = LspService::new(|client: Client| Backend { client });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_error_for_invalid_source() {
        let diags = syntax_diagnostics("1");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].message, "invalid token");
        assert_eq!(diags[0].range.start.line, 0);
        assert_eq!(diags[0].range.start.character, 0);
    }

    #[test]
    fn diagnostics_empty_for_valid_source() {
        let diags = syntax_diagnostics("abc");
        assert!(diags.is_empty());
    }
}
