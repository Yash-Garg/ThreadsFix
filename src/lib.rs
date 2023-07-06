mod thread;

use worker::*;

const BASE_URL: &str = "https://www.threads.net";

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _: RouteContext<()>| {
            Response::ok(
                "Fixing the internet, one thread at a time. Go to /t/<thread_id> to get started.",
            )
        })
        .get_async("/t/:thread_id", |_req, ctx| async move {
            let thread_id = ctx.param("thread_id").unwrap();

            Response::ok(format!("{}/t/{}", BASE_URL, thread_id))
        })
        .run(req, env)
        .await
}
