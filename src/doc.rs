use std::collections::HashMap;

use async_trait::async_trait;

pub trait OrgDoc {
    fn content(&self) -> &str;
}

#[derive(Debug)]
pub struct LazyDoc {
    pub path: String,
}

#[async_trait]
pub trait OrgSource: Send + Sync {
    async fn list(&self) -> Vec<LazyDoc>;
    async fn read(&self, doc: &LazyDoc) -> &dyn OrgDoc;
}

pub(crate) struct StaticOrgDoc(pub &'static str);

impl OrgDoc for StaticOrgDoc {
    fn content(&self) -> &str {
        self.0
    }
}

#[derive(Default)]
pub struct StaticOrgSource(HashMap<String, &'static str>);

impl StaticOrgSource {
    pub fn add_doc(&mut self, name: &str, content: &'static str) {
        self.0.insert(name.to_string(), content);
    }
}

#[async_trait]
impl OrgSource for StaticOrgSource {
    async fn list(&self) -> Vec<LazyDoc> {
        self.0.keys().map(|name| LazyDoc{ path: name.clone() }).collect()
    }

    async fn read(&self, _doc: &LazyDoc) -> &dyn OrgDoc {
        todo!()
    }
}
