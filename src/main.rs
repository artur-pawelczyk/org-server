mod doc;
mod empty_doc;
mod fs_doc;
mod render;
pub mod server;

use std::path::Path;

use fs_doc::FilesystemSource;
use server::Server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = FilesystemSource::new(Path::new("/home/artur/org"));

    let server = Server{ port: 8080 };
    server.start(Box::new(source)).await?;

    Ok(())
}
