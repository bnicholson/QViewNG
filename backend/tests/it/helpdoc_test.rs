use actix_web::{test, App, http::{header, StatusCode}};
use backend::routes::configure_routes;
use sha3::{Digest, Sha3_256};
use uuid::Uuid;
use crate::common::make_token;

// These endpoints touch no database, so the app is built without one.

const ARTICLE_URI: &str = "/api/helpdocs/qview/0.1.0.0/monitoring-rooms";

/// The signature the server puts on an image link, computed independently from the dev secret the
/// tests run with (as `make_token` uses).
fn image_link_sig(path: &str, exp: i64) -> String {
    let key = Sha3_256::new()
        .chain_update(b"qview-helpdocs-image-link-v1:")
        .chain_update(b"qview_dev_secret_changeme")
        .finalize();
    Sha3_256::new()
        .chain_update(key)
        .chain_update(path.as_bytes())
        .chain_update(b"|")
        .chain_update(exp.to_string().as_bytes())
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Every `src="..."` link in an article's HTML, with `&amp;` decoded as a browser would.
fn image_links(html: &str) -> Vec<String> {
    html.split("src=\"")
        .skip(1)
        .filter_map(|s| s.split('"').next())
        .map(|s| s.replace("&amp;", "&"))
        .collect()
}

fn member_token() -> String {
    make_token(Uuid::new_v4(), vec!["member".into()], vec!["tournament:read".into()])
}

#[actix_web::test]
async fn read_article_requires_login() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri(ARTICLE_URI).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let req = test::TestRequest::get()
        .uri(ARTICLE_URI)
        .insert_header((header::AUTHORIZATION, "Bearer not-a-real-token"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn read_article_works_for_any_logged_in_user() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // A member, and a user holding no roles or permissions at all.
    for token in [member_token(), make_token(Uuid::new_v4(), vec![], vec![])] {
        let req = test::TestRequest::get()
            .uri(ARTICLE_URI)
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers().get(header::CACHE_CONTROL).unwrap(), "private, no-store");

        let body: serde_json::Value = test::read_body_json(resp).await;
        let html = body["html"].as_str().expect("html");
        assert!(html.contains("Server Monitor"));
        // Screenshots point at signed links, never at the public /docs folder.
        let links = image_links(html);
        assert!(!links.is_empty());
        assert!(links.iter().all(|l| l.starts_with("/api/helpdocs/qview/0.1.0.0/images/") && l.contains("&sig=")));
    }
}

#[actix_web::test]
async fn read_article_returns_404_for_unknown_or_unsafe_paths() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    for uri in [
        "/api/helpdocs/qview/0.1.0.0/no-such-article",
        "/api/helpdocs/qview/9.9.9/monitoring-rooms",
        "/api/helpdocs/nope/0.1.0.0/monitoring-rooms",
        "/api/helpdocs/%2e%2e/0.1.0.0/monitoring-rooms",
        "/api/helpdocs/qview/0.1.0.0/Monitoring-Rooms",
    ] {
        let req = test::TestRequest::get()
            .uri(uri)
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", member_token())))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND, "{uri}");
    }
}

/// Every members-only article on disk serves, and every screenshot link in it loads. Catches an
/// article that references an image missing from its images/ folder.
#[actix_web::test]
async fn every_article_and_screenshot_loads() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("help-docs");

    let mut articles = 0;
    let mut images = 0;
    for category in std::fs::read_dir(&root).unwrap() {
        let category = category.unwrap().path();
        for version in std::fs::read_dir(&category).unwrap() {
            let version = version.unwrap().path();
            for file in std::fs::read_dir(&version).unwrap() {
                let file = file.unwrap().path();
                if file.extension().and_then(|e| e.to_str()) != Some("html") {
                    continue;
                }
                let uri = format!(
                    "/api/helpdocs/{}/{}/{}",
                    category.file_name().unwrap().to_str().unwrap(),
                    version.file_name().unwrap().to_str().unwrap(),
                    file.file_stem().unwrap().to_str().unwrap(),
                );
                let req = test::TestRequest::get()
                    .uri(&uri)
                    .insert_header((header::AUTHORIZATION, format!("Bearer {}", member_token())))
                    .to_request();
                let resp = test::call_service(&app, req).await;
                assert_eq!(resp.status(), StatusCode::OK, "{uri}");
                articles += 1;

                let body: serde_json::Value = test::read_body_json(resp).await;
                for link in image_links(body["html"].as_str().unwrap()) {
                    // No access token: the signed link alone is enough.
                    let resp = test::call_service(&app, test::TestRequest::get().uri(&link).to_request()).await;
                    assert_eq!(resp.status(), StatusCode::OK, "{link} (in {uri})");
                    assert!(resp.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap().starts_with("image/"));
                    assert!(!test::read_body(resp).await.is_empty());
                    images += 1;
                }
            }
        }
    }
    assert!(articles > 0 && images > 0);
}

#[actix_web::test]
async fn read_image_rejects_altered_or_expired_links() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let path = "qview/0.1.0.0/images/monitoring-rooms-2.webp";
    let now = chrono::Utc::now().timestamp();
    let exp = now + 3600;
    let sig = image_link_sig(path, exp);

    // The untouched link works (so the failures below are down to what was changed).
    let ok = format!("/api/helpdocs/{path}?exp={exp}&sig={sig}");
    let resp = test::call_service(&app, test::TestRequest::get().uri(&ok).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let other_image = "qview/0.1.0.0/images/monitoring-rooms-4.webp";
    let expired = now - 10;
    for uri in [
        // Expiry pushed out, signature kept.
        format!("/api/helpdocs/{path}?exp={}&sig={sig}", exp + 3600),
        // Signature for one image used on another.
        format!("/api/helpdocs/{other_image}?exp={exp}&sig={sig}"),
        // Signature altered.
        format!("/api/helpdocs/{path}?exp={exp}&sig={}", "0".repeat(64)),
        // Validly signed, but expired.
        format!("/api/helpdocs/{path}?exp={expired}&sig={}", image_link_sig(path, expired)),
    ] {
        let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN, "{uri}");
    }

    // No signature at all.
    let resp = test::call_service(&app, test::TestRequest::get().uri(&format!("/api/helpdocs/{path}")).to_request()).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // A validly signed link to a file that isn't there.
    let missing = "qview/0.1.0.0/images/no-such-image.webp";
    let uri = format!("/api/helpdocs/{missing}?exp={exp}&sig={}", image_link_sig(missing, exp));
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
