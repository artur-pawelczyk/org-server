use async_trait::async_trait;

use crate::doc::{OrgSource, OrgDoc};

pub struct EmptyOrgSource;
pub struct EmptyDoc;

#[async_trait]
impl OrgSource for EmptyOrgSource {
    async fn list(&self) -> Vec<String> {
        vec![]
    }

    async fn read(&self, _: &str) -> &dyn OrgDoc {
        &EmptyDoc
    }
}

impl OrgDoc for EmptyDoc {
    fn content(&self) -> &str {
        ""
    }
}
