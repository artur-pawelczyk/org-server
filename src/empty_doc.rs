use async_trait::async_trait;

use crate::doc::{OrgSource, OrgDoc, LazyDoc};

pub struct EmptyOrgSource;
pub struct EmptyDoc;

#[async_trait]
impl OrgSource for EmptyOrgSource {
    type Doc = EmptyDoc;

    async fn list(&self) -> Vec<LazyDoc<Self::Doc>> {
        vec![]
    }

    async fn read(&self, _: &LazyDoc<Self::Doc>) -> Self::Doc {
        EmptyDoc
    }
}

impl OrgDoc for EmptyDoc {
    fn content(&self) -> &str {
        ""
    }
}
