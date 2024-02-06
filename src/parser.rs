use std::{collections::HashMap, sync::Arc};

use orgize::Org;

#[derive(Debug)]
pub struct TodoItem<'a> {
    level: usize,
    keyword: Arc<str>,
    heading: &'a str,
}

impl<'a> TodoItem<'a> {
    pub fn keyword(&self) -> &str {
        self.keyword.as_ref()
    }

    pub fn heading(&self) -> &str {
        self.heading
    }

    fn level(&self) -> usize {
        self.level
    }
}

#[derive(Debug)]
enum KeywordState {
    Todo,
    Completed,
}

#[derive(Debug)]
pub struct ParserConfig {
    keywords: HashMap<Arc<str>, KeywordState>,
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

        let mut keywords = HashMap::new();
        keywords.extend(todo.iter().map(|s| (Arc::from(*s), KeywordState::Todo)));
        keywords.extend(done.iter().map(|s| (Arc::from(*s), KeywordState::Completed)));

        ParserConfig{ delegate, keywords }
    }

    fn as_org_config(&self) -> &orgize::ParseConfig {
        &self.delegate
    }

    fn intern_keyword(&self, keyword: &str) -> Option<Arc<str>> {
        self.keywords.get_key_value(keyword).map(|(k, _)| k).map(Arc::clone)
    }
}

pub fn doc_to_items(doc: &str, config: &ParserConfig, mut consumer: impl FnMut(TodoItem)) {
    let parsed = Org::parse_custom(doc, config.as_org_config());
    for headline in parsed.headlines() {
        let title = headline.title(&parsed);
        title.keyword.as_ref()
            .and_then(|keyword| config.intern_keyword(keyword))
            .map(|keyword| {
                consumer(TodoItem{
                    level: headline.level(),
                    keyword,
                    heading: title.raw.as_ref(),
                })
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_with_no_headings() {
        let doc = "";
        let mut items = Vec::new();
        doc_to_items(doc, &Default::default(), |_| items.push(true));
        assert_eq!(items.len(), 0);

        let doc = "some free content"; 
        let mut items = Vec::new();
        doc_to_items(doc, &Default::default(), |_| items.push(true));
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_doc_with_todo_headings() {
        let doc = "
* TODO First task
* TODO Second task";

        let mut items = Vec::new();
        doc_to_items(doc, &Default::default(), |item| items.push((item.level, item.keyword.clone(), item.heading.to_string())));

        assert_eq!(items.len(), 2);

        assert_eq!(items[0].0, 1);
        assert_eq!(items[0].1.as_ref(), "TODO");
        assert_eq!(items[0].2, "First task");

        assert_eq!(items[1].0, 1);
        assert_eq!(items[1].1.as_ref(), "TODO");
        assert_eq!(items[1].2, "Second task");
    }

    #[test]
    fn test_custom_keywords() {
        let doc = "
* NEW First task
* NEXT Second task";
        let config = ParserConfig::with_keywords(&["NEW", "NEXT"], &[]);

        let mut items = Vec::new();
        doc_to_items(doc, &config, |item| items.push(item.keyword.clone()));

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].as_ref(), "NEW");
        assert_eq!(items[1].as_ref(), "NEXT");
        
    }
}
