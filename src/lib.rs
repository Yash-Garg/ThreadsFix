#![allow(dead_code)]
mod client;
mod template;
mod types;
use types::KvCookie;
use worker::*;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .get("/", |_, _: RouteContext<()>| {
            Response::ok(
                "Fixing the internet, one thread at a time. Go to /t/<thread_id> to get started.",
            )
        })
        .get_async("/t/:thread_id", |_, ctx| async move {
            let thread_id = ctx.param("thread_id").unwrap();
            let kv = ctx.kv("COOKIE_STORE")?;
            let cookies = kv.get("key").text().await.unwrap().unwrap();
            let cookies: Vec<KvCookie> = serde_json::from_str(cookies.as_str()).unwrap();

            client::handle_thread_request(thread_id, cookies).await
        })
        .get_async("/t/:thread_id/", |_, ctx| async move {
            let thread_id = ctx.param("thread_id").unwrap();
            let kv = ctx.kv("COOKIE_STORE")?;
            let cookies = kv.get("key").text().await.unwrap().unwrap();
            let cookies: Vec<KvCookie> = serde_json::from_str(cookies.as_str()).unwrap();

            client::handle_thread_request(thread_id, cookies).await
        })
        .run(req, env)
        .await
}
