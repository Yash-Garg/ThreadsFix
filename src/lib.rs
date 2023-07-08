#![allow(dead_code)]
mod client;
mod template;
mod types;

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
            client::handle_thread_request(thread_id).await
        })
        .get_async("/t/:thread_id/", |_, ctx| async move {
            let thread_id = ctx.param("thread_id").unwrap();
            client::handle_thread_request(thread_id).await
        })
        .run(req, env)
        .await
}
