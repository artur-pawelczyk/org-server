use std::path::Path;

use async_trait::async_trait;
use org_server::doc::{OrgSource, OrgDoc};
use tokio::{fs::{read_dir, File as AsyncFile}, io::AsyncReadExt};
use tokio_stream::wrappers::ReadDirStream;
use futures_util::stream::StreamExt;

struct FilesystemSource<'a>(&'a Path);

struct FilesystemDoc(String);

impl OrgDoc for FilesystemDoc {
    fn content(&self) -> &str {
        &self.0
    }
}

impl<'a> FilesystemSource<'a> {
    pub fn new(path: &'a Path) -> Self {
        assert!(path.is_absolute());
        Self(path.as_ref())
    }
}

#[async_trait]
impl OrgSource for FilesystemSource<'_> {
    type Doc = FilesystemDoc;

    async fn list(&self) -> Vec<String> {
        if let Ok(contents) = read_dir(self.0).await {
            let mut res = Vec::new();
            let mut stream = ReadDirStream::new(contents);
            while let Some(Ok(file)) = stream.next().await {
                res.push(file.path().as_os_str().to_str().unwrap().to_string());
            }

            res
        } else {
            Vec::new()
        }
    }

    async fn read(&self, doc: &str) -> Result<Self::Doc, ()> {
        let path = Path::new(doc);
        if path.starts_with(self.0) {
            let mut content = String::new();
            AsyncFile::open(path).await.map_err(|_| ())?
                .read_to_string(&mut content).await.map_err(|_| ())?;
            Ok(FilesystemDoc(content))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, collections::BTreeSet};
    use std::io::Write;

    use super::*;
    use tempfile::tempdir;

    macro_rules! path_str {
        ($first:expr, $second:expr) => {
            {
                let full = $first.join($second).as_os_str().to_str().unwrap().to_string();
                Box::leak(Box::new(full))
            }
        };
    }    

    #[tokio::test]
    async fn test_no_files_found() {
        let dir = tempdir().unwrap();
        let source = FilesystemSource::new(dir.path());
        assert!(source.list().await.is_empty());
    }

    #[tokio::test]
    async fn test_some_files_found() {
        let dir = tempdir().unwrap();
        let files: BTreeSet<String> = ["tasks.org", "events.org"].iter()
            .map(|name| {
                File::create(dir.path().join(name)).unwrap();
                String::from(dir.path().join(name).as_os_str().to_str().unwrap())
            }).collect();

        let source = FilesystemSource::new(dir.path());
        let docs: BTreeSet<String> = source.list().await.iter().cloned().collect();
        assert_eq!(docs, files);
    }

    #[tokio::test]
    async fn test_file_contents() {
        let dir = tempdir().unwrap();
        let tasks_file_path = dir.path().join("tasks.org").as_os_str().to_str().unwrap().to_string();
        let mut tasks_file = File::create(dir.path().join("tasks.org")).unwrap();
        write!(tasks_file, "* Heading").unwrap();
        
        let source = FilesystemSource::new(dir.path());
        let doc = source.read(&tasks_file_path).await.unwrap();

        assert_eq!(doc.content(), "* Heading");
    }

    #[tokio::test]
    async fn test_file_not_available() {
        let source_dir = tempdir().unwrap();
        let other_dir = tempdir().unwrap();
        File::create(other_dir.path().join("tasks.org")).unwrap();
        let source = FilesystemSource::new(source_dir.path());

        assert!(source.read(path_str!(source_dir.path(), "tasks.org")).await.is_err());
        assert!(source.read(path_str!(other_dir.path(), "tasks.org")).await.is_err());
    }
}
