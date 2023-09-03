use std::path::Path;

use async_trait::async_trait;
use tokio::{fs::{read_dir, File as AsyncFile}, io::AsyncReadExt};
use tokio_stream::wrappers::ReadDirStream;
use futures_util::stream::StreamExt;

use crate::doc::{OrgDoc, OrgSource};

pub struct FilesystemSource<'a>(&'a Path);

pub struct FilesystemDoc(String);

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
                if file.path().extension().map(|ext| ext == "org").unwrap_or(false) {
                    if let Some(filename) = file.path().file_name().and_then(|s| s.to_str()) {
                        res.push(format!("/{filename}"));
                    }
                }
            }

            res
        } else {
            Vec::new()
        }
    }

    async fn read(&self, doc: &str) -> Result<Self::Doc, ()> {
        let doc = Path::new(doc).file_name().ok_or(())?;
        let path = self.0.join(doc);
        let mut content = String::new();
        AsyncFile::open(path).await.map_err(|_| ())?
            .read_to_string(&mut content).await.map_err(|_| ())?;

        Ok(FilesystemDoc(content))
    }

    fn doc_name(&self, doc: &str) -> String {
        Path::new(doc).file_name()
            .map(|s| s.to_str().expect("Path has to be a valid string").to_string())
            .expect("Has to be a vaild name")
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

    macro_rules! set {
        ($($x:expr),*) => {
            {
                let mut s: BTreeSet<String> = std::collections::BTreeSet::new();
                $(
                    s.insert($x.to_string());
                )*
                s
            }
        }
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
        make_files(dir.path(), &["tasks.org", "events.org"]);
        make_files(dir.path(), &["tasks.pdf", "events.pdf"]);

        let source = FilesystemSource::new(dir.path());
        let docs: BTreeSet<String> = source.list().await.iter().cloned().collect();
        assert_eq!(docs, set!("/tasks.org", "/events.org"));
    }

    #[tokio::test]
    async fn test_file_contents() {
        let dir = tempdir().unwrap();
        let mut tasks_file = File::create(dir.path().join("tasks.org")).unwrap();
        write!(tasks_file, "* Heading").unwrap();

        let source = FilesystemSource::new(dir.path());
        let doc = source.read("/tasks.org").await.unwrap();

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

    fn make_files(root: &Path, names: &[&str]) {
        for name in names {
            File::create(root.join(name)).unwrap();
        }
    }

    #[tokio::test]
    async fn test_doc_name() {
        let dir = tempdir().unwrap();
        make_files(dir.path(), &["tasks.org", "events.org"]);
        let source = FilesystemSource::new(dir.path());

        let files = source.list().await;
        let names = files.iter().map(|name| source.doc_name(name)).collect::<BTreeSet<String>>();
        assert_eq!(names, set!("tasks.org", "events.org"));
    }
}
