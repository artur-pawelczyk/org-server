use std::error::Error;

use axum::{Router, routing, extract::Path};
use config::Config;
use lazy_static::lazy_static;
use maud::{html, Markup, Render, PreEscaped, DOCTYPE};
use orgize::{Org, Event, Element};
use reqwest::Method;
use serde::Serialize;
use xml::{EventReader, reader::XmlEvent};

lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    static ref BASE_URL: String = CONFIG.get_string("base-url").unwrap();
    static ref USERNAME: String = CONFIG.get_string("username").unwrap();
    static ref PASSWORD: String = CONFIG.get_string("password").unwrap();
}

lazy_static! {
    static ref PROPFIND: Method = Method::from_bytes("PROPFIND".as_bytes()).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", routing::get(list_files_hander))
        .route("/todo", routing::get(find_todo_entries))
        .route("/:name", routing::get(render_org_doc));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn list_files_hander() -> Markup {
    let files = list_files().await.unwrap();

    html! {
        h1 { "Files" }
        a href="/todo" { "todos" }
        ul {
            @for file in &files {
                li {
                    (file)
                }
            }
        }
    }
}

async fn render_org_doc(Path(name): Path<String>) -> Markup {
    let full_path = format!("remote.php/dav/files/{}/org/{}", USERNAME.as_str(), name);
    let raw_file = read_file(&full_path).await.unwrap();
    let output = org_to_html(&raw_file).unwrap();
    html! {
        (DOCTYPE)
        (PreEscaped(output))
    }
}

fn org_to_html(org: &str) -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::with_capacity(1024);
    Org::parse(org).write_html(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

async fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let text = client.get(format!("{}/{}", BASE_URL.as_str(), path))
        .basic_auth(USERNAME.as_str(), Some(PASSWORD.as_str()))
        .send().await?
        .text().await?;

    Ok(text)
}

struct FileLink {
    name: String,
}

impl Render for FileLink {
    fn render(&self) -> Markup {
        html! {
            a href={ "/" (self.name) } { (self.name) }
        }
    }
}

async fn list_files() -> Result<Vec<FileLink>, Box<dyn Error>> {
    let url = BASE_URL.to_string() + "/remote.php/dav/files/" + &USERNAME + "/org";

    let client = reqwest::Client::new();
    let resp = client.request(PROPFIND.clone(), url)
        .basic_auth(USERNAME.as_str(), Some(PASSWORD.as_str()))
        .send()
        .await?
        .text()
        .await?;

    let mut xml_reader = EventReader::from_str(&resp).into_iter();
    let mut files = Vec::new();
    while let Some(e) = xml_reader.next() {
        match e? {
            XmlEvent::StartElement { name, .. } if name.local_name == "href" => {
                match xml_reader.next() {
                    Some(Ok(XmlEvent::Characters(path))) => {
                        if path.ends_with(".org") {
                            let name = path.split('/').last().unwrap().to_string();
                            files.push(FileLink{ name });
                        }
                    },
                    _ => panic!()
                }
            },
            _ => {}
        }
    }

    Ok(files)
}

#[derive(Serialize)]
struct TodoEntry {
    level: usize,
    keyword: String,
    summary: String,
}

// TODO: Error handling; remove 'unwrap'
async fn find_todo_entries() -> Markup {
    let full_path = format!("remote.php/dav/files/{}/org/se.org", USERNAME.as_str());
    let parser_conf = orgize::ParseConfig{
        todo_keywords: (
            ["NEW", "NEXT", "SOME", "WAIT", "PROJ"].iter().map(|s| s.to_string()).collect(),
            ["DONE", "CLND"].iter().map(|s| s.to_string()).collect(),
    )};
    let org = Org::parse_string_custom(read_file(&full_path).await.unwrap(), &parser_conf);

    let titles = org.iter()
        .map(|event| dbg!(event))
        .flat_map(|event| {
            match event {
                Event::Start(Element::Title(title)) => {
                    if title.keyword.is_some() {
                        Some(title)
                    } else {
                        None
                    }
                },
                _ => None
            }
        });

    html! {
        @for _title in titles {
            div { "some title" }
        }
    }
}
