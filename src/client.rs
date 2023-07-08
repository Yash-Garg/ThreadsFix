use std::{collections::HashMap, sync::Arc};

use askama::Template;
use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client, Url,
};
use serde_json::json;
use worker::{console_log, Response};

use crate::{
    template::IndexTemplate,
    types::{InnerThread, KvCookie},
};

const GQL_URL: &str = "https://www.threads.net/api/graphql";

pub async fn handle_thread_request(
    thread_id: &str,
    cookies: Vec<KvCookie>,
) -> worker::Result<Response> {
    let token = get_token().await;
    let thread_url = format!("https://www.threads.net/t/{}", thread_id);

    let jar = reqwest::cookie::Jar::default();
    for cookie in cookies.iter() {
        jar.add_cookie_str(
            KvCookie::to_cookie_crate(cookie.clone())
                .to_string()
                .as_str(),
            &Url::parse(&thread_url).unwrap(),
        );
    }

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let post_id = get_post_id(thread_id.to_string(), client.clone()).await;
    let id = json!({ "postID": post_id }).to_string();

    let mut form_data: HashMap<&str, &str> = HashMap::new();
    form_data.insert("lsd", &token.as_str());
    form_data.insert("variables", &id);
    form_data.insert("doc_id", "6529829603744567");

    console_log!("form_data: {:#?}", form_data);

    let thread = client
        .post(GQL_URL)
        .header(USER_AGENT, "threads-fix")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("X-IG-App-ID", "238260118697367")
        .header("X-FB-LSD", &token)
        .header("Sec-Fetch-Site", "same-origin")
        .form(&form_data)
        .send()
        .await;

    match thread {
        Ok(response) => {
            let body = response.text().await.unwrap();
            let data: InnerThread = serde_json::from_str(&body.as_str()).unwrap();
            let post = &data.data.data.containing.items.first().unwrap().post;

            let is_media = post.media.candidates.len() > 0;

            let template = IndexTemplate {
                title: format!("{} on Threads", &post.user.username),
                image: is_media
                    .then(|| post.media.candidates.first().unwrap().url.clone())
                    .unwrap_or_else(|| String::from("")),
                description: String::from(&post.caption.text),
                url: thread_url.to_string(),
                width: post.original_width,
                height: post.original_height,
                video: String::from(""),
            };

            let rendered = template.render().unwrap();
            Response::from_html(rendered)
        }
        Err(_) => Response::redirect(Url::parse(&thread_url).unwrap()),
    }
}

async fn get_token() -> String {
    let client = reqwest::Client::builder().build().unwrap();

    let response = client
        .get("https://www.threads.net/@instagram")
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();

    let token = body
        .split("{\"token\":\"")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split("\"")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .to_string();

    console_log!("token: {}", &token);

    token
}

async fn get_post_id(thread_id: String, client: Client) -> String {
    let response = client
        .get(format!("https://www.threads.net/t/{}", thread_id))
        .header(USER_AGENT, "threads-fix")
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    let post_id = body
        .split("{\"post_id\":\"")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split("\"")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .to_string();

    post_id
}
