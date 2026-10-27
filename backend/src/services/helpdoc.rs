//! Members-only Help Center articles.
//!
//! Most Help Center articles ship inside the frontend bundle, readable by anyone. The ones a
//! visitor must log in to read live here instead, in `help-docs/{category}/{version}/`, so their
//! text and screenshots never leave the server for an anonymous request:
//!
//!   {slug}.html        an article's HTML, with screenshots referenced by bare file name
//!                      (`<img src="monitoring-rooms-2.webp">`)
//!   images/{file}      those screenshots
//!
//! The article endpoint requires a logged-in user (any one; no particular permission). An `<img>`
//! can't send the access token, so the article's screenshot links are rewritten into signed,
//! expiring links that the image endpoint checks instead.

use actix_web::{get, http::header, HttpMessage, HttpRequest, HttpResponse, web::{Path, Query}};
use chrono::Utc;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::auth::{jwt_secret, policies::UserContext};

#[derive(RustEmbed)]
#[folder = "help-docs/"]
struct HelpDocs;

/// Image links stay valid for at least this long, and at most twice it. Expiry is rounded up to a
/// window boundary so that every load of an article within one window produces the same links,
/// which keeps the browser's image cache useful.
const IMAGE_LINK_WINDOW_SECS: i64 = 60 * 60;

#[derive(Serialize)]
struct HelpDocResponse {
    html: String,
}

#[derive(Deserialize)]
struct ImageLinkQuery {
    exp: i64,
    sig: String,
}

/// A path segment safe to join into an embedded-file path: lowercase letters, digits, `-` and `.`,
/// and not starting with `.` (so never `..`).
fn is_safe_segment(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('.')
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
}

/// Signing key for image links, derived from the access-token secret under its own label so the
/// two uses never share a key.
fn image_link_key() -> Vec<u8> {
    let mut h = Sha3_256::new();
    h.update(b"qview-helpdocs-image-link-v1:");
    h.update(jwt_secret().as_bytes());
    h.finalize().to_vec()
}

/// SHA3-256(key ‖ path ‖ exp) as hex. SHA-3 is not open to length extension, so a keyed hash is a
/// sound MAC here without an HMAC construction.
fn sign_image_link(path: &str, exp: i64) -> String {
    let mut h = Sha3_256::new();
    h.update(image_link_key());
    h.update(path.as_bytes());
    h.update(b"|");
    h.update(exp.to_string().as_bytes());
    h.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}

/// Compares in time independent of where the inputs first differ.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Rewrites every `src="{file}"` that names a bare file (no `/` or `:`) into a signed link to the
/// image endpoint. Anything else (absolute paths, external URLs) is left as authored.
fn sign_image_links(html: &str, category: &str, version: &str, now: i64) -> String {
    let exp = (now / IMAGE_LINK_WINDOW_SECS + 2) * IMAGE_LINK_WINDOW_SECS;
    let mut out = String::with_capacity(html.len() + 4096);
    let mut rest = html;
    while let Some(i) = rest.find("src=\"") {
        let (before, after) = rest.split_at(i + 5);
        out.push_str(before);
        let Some(end) = after.find('"') else { rest = after; break };
        let file = &after[..end];
        if is_safe_segment(file) {
            let path = format!("{}/{}/images/{}", category, version, file);
            out.push_str(&format!("/api/helpdocs/{}?exp={}&amp;sig={}", path, exp, sign_image_link(&path, exp)));
        } else {
            out.push_str(file);
        }
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

fn content_type_for(file: &str) -> Option<&'static str> {
    match file.rsplit_once('.')?.1 {
        "webp" => Some("image/webp"),
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        _ => None,
    }
}

/// A members-only article's HTML. 401 when not logged in; 404 for an article that isn't here.
#[get("/{category}/{version}/{slug}")]
async fn read_article(req: HttpRequest, path: Path<(String, String, String)>) -> HttpResponse {
    if req.extensions().get::<UserContext>().is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let (category, version, slug) = path.into_inner();
    if ![&category, &version, &slug].iter().all(|s| is_safe_segment(s)) {
        return HttpResponse::NotFound().finish();
    }
    let Some(file) = HelpDocs::get(&format!("{}/{}/{}.html", category, version, slug)) else {
        return HttpResponse::NotFound().finish();
    };
    let Ok(html) = std::str::from_utf8(&file.data) else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok()
        .insert_header((header::CACHE_CONTROL, "private, no-store"))
        .json(HelpDocResponse { html: sign_image_links(html, &category, &version, Utc::now().timestamp()) })
}

/// A screenshot from a members-only article. Needs the signed link the article was served with
/// rather than an access token; 403 when the link is altered or expired.
#[get("/{category}/{version}/images/{file}")]
async fn read_image(path: Path<(String, String, String)>, query: Query<ImageLinkQuery>) -> HttpResponse {
    let (category, version, file) = path.into_inner();
    if ![&category, &version, &file].iter().all(|s| is_safe_segment(s)) {
        return HttpResponse::NotFound().finish();
    }
    let link_path = format!("{}/{}/images/{}", category, version, file);
    let now = Utc::now().timestamp();
    let expected = sign_image_link(&link_path, query.exp);
    if query.exp <= now || !constant_time_eq(expected.as_bytes(), query.sig.as_bytes()) {
        return HttpResponse::Forbidden().finish();
    }
    let (Some(content_type), Some(image)) = (content_type_for(&file), HelpDocs::get(&link_path)) else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok()
        .content_type(content_type)
        .insert_header((header::CACHE_CONTROL, format!("private, max-age={}", query.exp - now)))
        .body(image.data.into_owned())
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(read_image)
        .service(read_article)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_segments() {
        for s in ["", ".", "..", ".hidden", "A", "a/b", "a\\b", "a b", "%2e%2e"] {
            assert!(!is_safe_segment(s), "{s:?} should be rejected");
        }
        for s in ["qview", "0.1.0.0", "monitoring-rooms", "monitoring-rooms-2.webp"] {
            assert!(is_safe_segment(s), "{s:?} should be accepted");
        }
    }

    #[test]
    fn signs_only_bare_image_names() {
        let html = r#"<img src="a-1.webp" alt="x" /><img src="/docs/qview/b.webp" /><img src="https://e.com/c.png" />"#;
        let out = sign_image_links(html, "qview", "0.1.0.0", 0);
        let exp = 2 * IMAGE_LINK_WINDOW_SECS;
        let sig = sign_image_link("qview/0.1.0.0/images/a-1.webp", exp);
        assert!(out.contains(&format!(r#"src="/api/helpdocs/qview/0.1.0.0/images/a-1.webp?exp={exp}&amp;sig={sig}""#)));
        assert!(out.contains(r#"src="/docs/qview/b.webp""#));
        assert!(out.contains(r#"src="https://e.com/c.png""#));
        assert!(out.ends_with("/>"));
    }

    #[test]
    fn links_are_stable_within_a_window() {
        let a = sign_image_links(r#"<img src="a.webp">"#, "qview", "1", 10);
        let b = sign_image_links(r#"<img src="a.webp">"#, "qview", "1", IMAGE_LINK_WINDOW_SECS - 1);
        assert_eq!(a, b);
    }

    #[test]
    fn signature_binds_path_and_expiry() {
        let s = sign_image_link("qview/1/images/a.webp", 100);
        assert_ne!(s, sign_image_link("qview/1/images/b.webp", 100));
        assert_ne!(s, sign_image_link("qview/1/images/a.webp", 101));
    }
}
