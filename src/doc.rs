
use std::{collections::HashMap, path::Path};


use async_trait::async_trait;

pub trait OrgDoc {
    fn content(&self) -> &str;
}

#[async_trait]
pub trait OrgSource: Send + Sync {
    type Doc: OrgDoc;

    async fn list(&self) -> Vec<String>;
    async fn read(&self, doc: &str) -> Result<Self::Doc, ()>;

    fn doc_name(&self, doc: &str) -> String {
        String::from(doc)
    }
}

#[derive(Clone)]
pub struct StaticOrgDoc(pub &'static str);

impl OrgDoc for StaticOrgDoc {
    fn content(&self) -> &str {
        self.0
    }
}

#[derive(Default)]
pub struct StaticOrgSource(HashMap<String, StaticOrgDoc>);

impl StaticOrgSource {
    #[allow(dead_code)]
    pub fn add_doc(&mut self, name: &str, content: &'static str) {
        self.0.insert(name.to_string(), StaticOrgDoc(content));
    }
}

#[async_trait]
impl OrgSource for StaticOrgSource {
    type Doc = StaticOrgDoc;

    async fn list(&self) -> Vec<String> {
        self.0.keys().map(|key| format!("/{key}")).collect()
    }

    async fn read(&self, doc: &str) -> Result<StaticOrgDoc, ()> {
        let path = Path::new(doc).file_name().ok_or(())?;
        let doc = path.to_str().ok_or(())?;
        self.0.get(doc).cloned().ok_or(())
    }
}
