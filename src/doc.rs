use std::{marker::PhantomData, collections::HashMap};

use async_trait::async_trait;

pub trait OrgDoc {
    fn content(&self) -> &str;
}

pub struct LazyDoc<T: OrgDoc> {
    pub path: String,
    _marker: PhantomData<T>,
}

#[async_trait]
pub trait OrgSource {
    type Doc: OrgDoc;

    async fn list(&self) -> Vec<LazyDoc<Self::Doc>>;
    async fn read(&self, doc: &LazyDoc<Self::Doc>) -> Self::Doc;
}

pub(crate) struct StaticOrgDoc(pub &'static str);

impl OrgDoc for StaticOrgDoc {
    fn content(&self) -> &str {
        self.0
    }
}

#[derive(Default)]
pub(crate) struct StaticOrgSource(HashMap<String, &'static str>);

impl StaticOrgSource {
    pub(crate) fn add_doc(&mut self, name: &str, content: &'static str) {
        self.0.insert(name.to_string(), content);
    }
}

#[async_trait]
impl OrgSource for StaticOrgSource {
    type Doc = StaticOrgDoc;

    async fn list(&self) -> Vec<LazyDoc<Self::Doc>> {
        self.0.keys().map(|name| LazyDoc{ path: name.clone(), _marker: PhantomData }).collect()
    }

    async fn read(&self, _doc: &LazyDoc<Self::Doc>) -> Self::Doc {
        todo!()
    }
}
