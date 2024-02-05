use std::{collections::HashSet, sync::Arc};

use orgize::Org;

#[derive(Debug)]
pub struct TodoItem(usize, Arc<str>, String);

impl TodoItem {
    pub fn keyword(&self) -> &str {
        &self.1
    }

    pub fn heading(&self) -> &str {
        &self.2
    }

    fn level(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct ParserConfig {
    keywords: HashSet<(Arc<str>, bool)>,
    delegate: orgize::ParseConfig,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self::with_keywords(&["TODO"], &["DONE"])
    }
}

impl ParserConfig {
    pub fn with_keywords(todo: &[&str], done: &[&str]) -> Self {
        let delegate = orgize::ParseConfig{ todo_keywords: (todo.iter().map(|s| String::from(*s)).collect(),
                                                            done.iter().map(|s| String::from(*s)).collect()) };

        let mut keywords = HashSet::new();
        keywords.extend(todo.iter().map(|s| (Arc::from(*s), false)));
        keywords.extend(done.iter().map(|s| (Arc::from(*s), true)));

        ParserConfig{ delegate, keywords }
    }

    fn as_org_config(&self) -> &orgize::ParseConfig {
        &self.delegate
    }

    fn intern_keyword(&self, keyword: &str) -> Option<Arc<str>> {
        self.keywords.iter()
            .find(|x| x.0.as_ref() == keyword)
            .map(|x| Arc::clone(&x.0))
    }
}

pub fn doc_to_items(doc: &str, config: &ParserConfig) -> Vec<TodoItem> {
    let parsed = Org::parse_custom(doc, config.as_org_config());
    parsed.headlines()
        .flat_map(|headline| {
            let title = headline.title(&parsed);
            title.keyword.as_ref()
                .and_then(|keyword| config.intern_keyword(keyword))
                .map(|keyword| {
                    TodoItem(headline.level(),
                             keyword,
                             title.raw.to_string())
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_with_no_headings() {
        let doc = "";
        let items = doc_to_items(doc, &Default::default()).len();
        assert_eq!(items, 0);

        let doc = "some free content"; 
        let items = doc_to_items(doc, &Default::default()).len();
        assert_eq!(items, 0);
    }

    #[test]
    fn test_doc_with_todo_headings() {
        let doc = "
* TODO First task
* TODO Second task";

        let items: Vec<_> = doc_to_items(doc, &Default::default());
        assert_eq!(items.len(), 2);

        assert_eq!(items[0].level(), 1);
        assert_eq!(items[0].keyword(), "TODO");
        assert_eq!(items[0].heading(), "First task");

        assert_eq!(items[1].level(), 1);
        assert_eq!(items[1].keyword(), "TODO");
        assert_eq!(items[1].heading(), "Second task");
    }

    #[test]
    fn test_custom_keywords() {
        let doc = "
* NEW First task
* NEXT Second task";
        let config = ParserConfig::with_keywords(&["NEW", "NEXT"], &[]);

        let items = doc_to_items(doc, &config);

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].keyword(), "NEW");
        assert_eq!(items[1].keyword(), "NEXT");
        
    }
}
