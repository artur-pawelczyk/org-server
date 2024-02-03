use orgize::Org;

#[derive(Debug)]
pub struct TodoItem(usize, String, String);

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

#[derive(Default, Debug)]
pub struct ParserConfig(orgize::ParseConfig);

impl ParserConfig {
    pub fn with_keywords(todo: &[&str], done: &[&str]) -> Self {
        Self(orgize::ParseConfig{ todo_keywords: (todo.iter().map(|s| String::from(*s)).collect(),
                                                  done.iter().map(|s| String::from(*s)).collect()) })
    }

    fn as_org_config(&self) -> &orgize::ParseConfig {
        &self.0
    }
}

pub fn doc_to_items(doc: &str, config: &ParserConfig) -> Vec<TodoItem> {
    let parsed = Org::parse_custom(doc, config.as_org_config());
    let out = parsed.headlines()
        .flat_map(|headline| {
            let title = headline.title(&parsed);
            title.keyword.as_ref().map(|keyword| {
            TodoItem(headline.level(),
                     keyword.to_string(),
                     title.raw.to_string())
            })
        })
        .collect();

    dbg!(out)
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
