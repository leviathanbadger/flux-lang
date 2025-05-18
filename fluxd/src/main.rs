use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidOpenTextDocumentParams, Position, Range,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use flux_lang::parse_program;

#[derive(Clone)]
struct Backend {
    client: Client,
}

fn extract_offset(msg: &str) -> Option<usize> {
    msg.rsplit(" at ")
        .next()
        .and_then(|s| s.parse::<usize>().ok())
}

fn offset_to_position(src: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in src.char_indices() {
        if i == offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Position::new(line, col)
}

fn syntax_diagnostics(text: &str) -> Vec<Diagnostic> {
    match parse_program(text) {
        Ok(_) => Vec::new(),
        Err(err) => {
            let msg = err.to_string();
            let position = extract_offset(&msg)
                .map(|o| offset_to_position(text, o))
                .unwrap_or_else(|| Position::new(0, 0));
            vec![Diagnostic {
                range: Range::new(position, position),
                severity: Some(DiagnosticSeverity::ERROR),
                message: msg,
                ..Default::default()
            }]
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
        assert!(diags[0].message.contains("parse error"));
    }

    #[test]
    fn diagnostics_empty_for_valid_source() {
        let diags = syntax_diagnostics("abc");
        assert!(diags.is_empty());
    }
}
