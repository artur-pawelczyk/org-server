use std::collections::HashMap;

use async_trait::async_trait;

pub trait OrgDoc {
    fn content(&self) -> &str;
}

#[async_trait]
pub trait OrgSource: Send + Sync {
    async fn list(&self) -> Vec<String>;
    async fn read(&self, doc: &str) -> &dyn OrgDoc;
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
    async fn list(&self) -> Vec<String> {
        self.0.keys().map(String::from).collect()
    }

    async fn read(&self, doc: &str) -> &dyn OrgDoc {
        self.0.get(doc).expect("It shouldn't fail because 'list' was called first")
    }
}
