use maud::{Markup, html, DOCTYPE, PreEscaped, Render};

#[derive(Default)]
pub struct Page;

impl Page {
    pub fn render(&self, inner: impl Render) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head { (inner) }
                body {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use scraper::{Html, Selector};

    use super::*;

    #[test]
    fn test_empty_page() {
        let page = Page::default();

        assert_eq!("<!DOCTYPE html><html><head></head><body></body></html>", page.render("").into_string());
    }

    #[test]
    fn test_page_with_string_content() {
        let page = Page::default();

        let output = page.render(PreEscaped("<h1>heading</h1>")).into_string();

        let html = Html::parse_document(&output);
        let selector = Selector::parse("body > h1").unwrap();
        assert_eq!(1, html.select(&selector).count());
    }

    #[test]
    fn test_page_with_markup_content() {
        let page = Page::default();

        let output = page.render(html! { h1 { "heading" } }).into_string();

        let html = Html::parse_document(&output);
        let selector = Selector::parse("body > h1").unwrap();
        assert_eq!(1, html.select(&selector).count());
    }
}
