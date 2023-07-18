use std::marker::PhantomData;

use async_trait::async_trait;

pub(crate) trait OrgDoc {
    fn content(&self) -> &str;
}

pub(crate) struct LazyDoc<T: OrgDoc> {
    _marker: PhantomData<T>,
}

#[async_trait]
pub(crate) trait OrgSource {
    type Doc: OrgDoc;

    async fn list(&self) -> Vec<LazyDoc<Self::Doc>>;
    async fn read(&self, doc: &LazyDoc<Self::Doc>) -> Self::Doc;
}

pub(crate) struct StaticOrgSource;
pub(crate) struct StaticOrgDoc(pub &'static str);

// impl StaticOrgDoc {
//     pub(crate) fn from(content: impl AsRef<str>) -> Self {
//         StaticOrgDoc(content.as_ref().to_string())
//     }
// }

impl OrgDoc for StaticOrgDoc {
    fn content(&self) -> &str {
        self.0
    }
}

#[async_trait]
impl OrgSource for StaticOrgSource {
    type Doc = StaticOrgDoc;

    async fn list(&self) -> Vec<LazyDoc<Self::Doc>> {
        todo!()
    }

    async fn read(&self, doc: &LazyDoc<Self::Doc>) -> Self::Doc {
        todo!()
    }
}
