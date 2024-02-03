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

pub fn doc_to_items(doc: &str) -> Vec<TodoItem> {
    let parsed = Org::parse(doc);
    parsed.headlines()
        .flat_map(|headline| {
            let title = headline.title(&parsed);
            title.keyword.as_ref().map(|keyword| {
            TodoItem(headline.level(),
                     keyword.to_string(),
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
        let items = doc_to_items(doc).len();
        assert_eq!(items, 0);

        let doc = "some free content"; 
        let items = doc_to_items(doc).len();
        assert_eq!(items, 0);
    }

    #[test]
    fn test_doc_with_todo_headings() {
        let doc = "
* TODO First task
* TODO Second task";

        let items: Vec<_> = doc_to_items(doc);
        assert_eq!(items.len(), 2);

        assert_eq!(items[0].level(), 1);
        assert_eq!(items[0].keyword(), "TODO");
        assert_eq!(items[0].heading(), "First task");

        assert_eq!(items[1].level(), 1);
        assert_eq!(items[1].keyword(), "TODO");
        assert_eq!(items[1].heading(), "Second task");
    }
}
