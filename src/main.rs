mod doc;
mod empty_doc;
mod fs_doc;
mod render;
mod server;
mod parser;
mod page;

use std::path::Path;

use fs_doc::FilesystemSource;
use parser::ParserConfig;
use server::Server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let org_files_path = Box::leak(Box::new(Path::new("examples/org").canonicalize()?));
    let source = FilesystemSource::new(org_files_path.as_path());

    let server = Server{
        port: 8080,
        parser_config: ParserConfig::with_keywords(&["NEW", "NEXT"], &["DONE"]),
    };
    server.start(source).await?;

    Ok(())
}
