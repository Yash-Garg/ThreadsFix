use cookie::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerThread {
    pub data: InnerThreadData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerThreadData {
    pub data: DataInner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInner {
    #[serde(rename = "containing_thread")]
    pub containing: Thread,
}

/// A thread of posts.
#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    #[serde(rename = "thread_items")]
    pub items: Vec<ThreadItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadItem {
    pub post: Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub code: String,
    pub user: User,
    #[serde(rename = "image_versions2")]
    pub media: Media,
    #[serde(rename = "like_count")]
    pub likes: u32,
    pub caption: Caption,
    pub taken_at: u64,
    pub original_width: u32,
    pub original_height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caption {
    pub text: String,
}

/// Contains the minimum required information to display a profile.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "pk")]
    pub id: String,
    pub profile_pic_url: String,
    pub username: String,
    pub is_verified: bool,
}

/// A media item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvCookie {
    pub domain: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "hostOnly")]
    pub host_only: bool,
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    pub name: String,
    pub path: String,
    #[serde(rename = "sameSite")]
    pub same_site: String,
    pub secure: bool,
    pub session: bool,
    #[serde(rename = "storeId")]
    pub store_id: String,
    pub value: String,
}

impl KvCookie {
    pub fn to_cookie_crate(cookie: KvCookie) -> Cookie<'static> {
        Cookie::build(cookie.name, cookie.value)
            .domain(cookie.domain)
            .path(cookie.path)
            .secure(cookie.secure)
            .http_only(cookie.http_only)
            .finish()
    }
}
