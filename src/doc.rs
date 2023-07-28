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
pub struct StaticOrgSource(HashMap<String, StaticOrgDoc>);

impl StaticOrgSource {
    #[allow(dead_code)]
    pub fn add_doc(&mut self, name: &str, content: &'static str) {
        self.0.insert(name.to_string(), StaticOrgDoc(content));
    }
}

#[async_trait]
impl OrgSource for StaticOrgSource {
    async fn list(&self) -> Vec<LazyDoc> {
        self.0.keys().map(|name| LazyDoc{ path: name.clone() }).collect()
    }

    async fn read(&self, doc: &LazyDoc) -> &dyn OrgDoc {
        self.0.get(&doc.path).expect("It shouldn't fail because 'list' was called first")
    }
}
