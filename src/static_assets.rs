use axum::body::Body;
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend/build"]
struct FrontendAssets;

/// 兜底处理所有非 /api 路由：
/// - 命中嵌入文件则直接返回
/// - 看起来像静态资源（含 `.` 扩展名 / SvelteKit `__data.json` / `/_app/`）→ 404
/// - 其余视作 SPA 页面路径，返回 index.html 由前端路由接管
pub async fn fallback(uri: Uri) -> Response {
    let raw = uri.path().trim_start_matches('/');

    if !raw.is_empty() {
        if let Some(resp) = serve(raw) {
            return resp;
        }
        if looks_like_asset(raw) {
            return (StatusCode::NOT_FOUND, "not found").into_response();
        }
    }

    match serve("index.html") {
        Some(resp) => resp,
        None => (StatusCode::NOT_FOUND, "frontend not built").into_response(),
    }
}

/// 路径看起来是资源（不该被 SPA 兜底）的启发式判断。
fn looks_like_asset(path: &str) -> bool {
    if path.starts_with("_app/") || path.contains("/_app/") {
        return true;
    }
    let last = path.rsplit('/').next().unwrap_or("");
    last == "__data.json" || last.contains('.')
}

fn serve(path: &str) -> Option<Response> {
    let file = FrontendAssets::get(path)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    Some(
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(file.data.into_owned()))
            .unwrap(),
    )
}
