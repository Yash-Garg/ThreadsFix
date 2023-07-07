use worker::{console_log, Response};

use crate::thread::{self, IndexTemplate};
use askama::Template;
use reqwest::{
    header::{
        HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, REFERER,
        UPGRADE_INSECURE_REQUESTS, USER_AGENT,
    },
    Url,
};

const BASE_URL: &str = "https://www.threads.net";

pub fn get_headers(thread_id: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let url = Url::parse(&format!("{}/t/{}", BASE_URL, thread_id)).unwrap();

    headers.insert(
        HeaderName::from_static("authority"),
        HeaderValue::from_static("www.threads.net"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
        ),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(REFERER, HeaderValue::from_str(&url.to_string()).unwrap());
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
        AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36",
        ),
    );
    headers.insert(
        HeaderName::from_static("viewport-width"),
        HeaderValue::from_static("1280"),
    );

    headers
}

pub async fn handle_thread_request(thread_id: &str) -> worker::Result<Response> {
    let client = reqwest::Client::builder()
        .default_headers(get_headers(&thread_id))
        .build()
        .unwrap();

    let url = format!("{}/t/{}", BASE_URL, thread_id);
    let thread = client.get(&url).send().await;

    match thread {
        Ok(response) => {
            let body = response.text().await.unwrap();
            let doc = scraper::Html::parse_document(&body);

            let meta = &doc
                .select(&scraper::Selector::parse("meta[name], meta[property]").unwrap())
                .map(|element| {
                    let name = element.value().attr("name");
                    let property = element.value().attr("property");
                    let content = element.value().attr("content");

                    match (name, property, content) {
                        (Some(name), _, Some(content)) => (name, content),
                        (_, Some(property), Some(content)) => (property, content),
                        _ => ("", ""),
                    }
                })
                .collect::<Vec<_>>();

            let thread = thread::Thread::parse(meta.to_vec());
            console_log!("Meta: {:?}", &meta);

            let template_model = IndexTemplate {
                title: thread.title,
                description: thread.description,
                image: thread.image,
                video: String::from(""),
                url: url.clone(),
                width: 1280,
                height: 720,
            };

            let body = template_model.render();

            match body {
                Ok(body) => Response::from_html(body),
                Err(_) => Response::redirect(Url::parse(&url).unwrap()),
            }
        }
        Err(_) => Response::error("Thread not found", 404),
    }
}
