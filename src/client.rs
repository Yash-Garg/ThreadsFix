use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use worker::{console_log, Response};

const GQL_URL: &str = "https://www.threads.net/api/graphql";

pub async fn handle_thread_request(thread_id: &str) -> worker::Result<Response> {
    let token = get_token().await;
    let headers = get_headers(&token);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let post_id = json!({ "postID": thread_id });

    let mut form_data: HashMap<&str, &str> = HashMap::new();
    form_data.insert("lsd", &token.as_str());
    form_data.insert("variables", post_id.as_str().unwrap());
    form_data.insert("doc_id", "5587632691339264");

    console_log!("form_data: {:#?}", form_data);
    console_log!("post_id: {:#?}", post_id);

    let thread = client.post(GQL_URL).form(&form_data).send().await;

    Response::ok(format!("{:#?}", thread))
}

fn get_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)",
        ),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        HeaderName::from_static("X-IG-App-ID"),
        HeaderValue::from_str(&token).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("X-FB-LSD"),
        HeaderValue::from_str(&token).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("Sec-Fetch-Site"),
        HeaderValue::from_static("same-origin"),
    );

    headers
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
