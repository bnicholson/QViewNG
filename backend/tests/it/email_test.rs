//! Integration tests for the SES-backed email service's configuration + link building.
//!
//! These exercise the real public API (`EmailConfig::from_source`, `EmailService::from_parts`,
//! `activation_link`/`reset_link`) with an in-memory key→value source, so they're deterministic and
//! never touch the network or the process environment. Actually delivering mail requires live SES
//! credentials and is out of scope for an automated test.

use std::collections::HashMap;

use backend::services::email::{EmailConfig, EmailService};

/// A `lookup` closure backed by a fixed map, standing in for the process environment.
fn source(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let map: HashMap<String, String> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    move |key: &str| map.get(key).cloned()
}

#[test]
fn from_source_reads_ses_vars_and_defaults_port_587() {
    let cfg = EmailConfig::from_source(source(&[
        ("SES_SMTP_HOST", "email-smtp.us-east-1.amazonaws.com"),
        ("SES_SMTP_USERNAME", "AKIAEXAMPLE"),
        ("SES_SMTP_PASSWORD", "secret"),
        ("SES_FROM_ADDRESS", "QView <no-reply@example.com>"),
    ])).expect("config should build from SES_* vars");

    assert_eq!(cfg.smtp_host, "email-smtp.us-east-1.amazonaws.com");
    assert_eq!(cfg.username, "AKIAEXAMPLE");
    assert_eq!(cfg.password, "secret");
    assert_eq!(cfg.from_address, "QView <no-reply@example.com>");
    assert_eq!(cfg.smtp_port, 587, "port should default to 587 (STARTTLS)");
}

#[test]
fn from_source_honors_explicit_port() {
    let cfg = EmailConfig::from_source(source(&[
        ("SES_SMTP_HOST", "email-smtp.us-west-2.amazonaws.com"),
        ("SES_SMTP_USERNAME", "u"),
        ("SES_SMTP_PASSWORD", "p"),
        ("SES_FROM_ADDRESS", "no-reply@example.com"),
        ("SES_SMTP_PORT", "465"),
    ])).unwrap();
    assert_eq!(cfg.smtp_port, 465);
}

#[test]
fn ses_vars_take_precedence_over_legacy_smtp_vars() {
    let cfg = EmailConfig::from_source(source(&[
        ("SES_SMTP_HOST", "ses-host"),
        ("SMTP_HOST", "legacy-host"),
        ("SES_SMTP_USERNAME", "ses-user"),
        ("SMTP_USER", "legacy-user"),
        ("SES_SMTP_PASSWORD", "ses-pass"),
        ("SES_FROM_ADDRESS", "ses@example.com"),
    ])).unwrap();
    assert_eq!(cfg.smtp_host, "ses-host");
    assert_eq!(cfg.username, "ses-user");
}

#[test]
fn falls_back_to_legacy_smtp_vars_when_ses_unset() {
    let cfg = EmailConfig::from_source(source(&[
        ("SMTP_HOST", "legacy-host"),
        ("SMTP_USER", "legacy-user"),
        ("SMTP_PASS", "legacy-pass"),
        ("SMTP_FROM", "legacy@example.com"),
    ])).unwrap();
    assert_eq!(cfg.smtp_host, "legacy-host");
    assert_eq!(cfg.username, "legacy-user");
    assert_eq!(cfg.password, "legacy-pass");
    assert_eq!(cfg.from_address, "legacy@example.com");
}

#[test]
fn blank_values_count_as_unset() {
    // A present-but-blank SES host must not shadow the legacy fallback.
    let cfg = EmailConfig::from_source(source(&[
        ("SES_SMTP_HOST", "   "),
        ("SMTP_HOST", "legacy-host"),
        ("SMTP_USER", "u"),
        ("SMTP_PASS", "p"),
        ("SMTP_FROM", "f@example.com"),
    ])).unwrap();
    assert_eq!(cfg.smtp_host, "legacy-host");
}

#[test]
fn missing_required_vars_error() {
    // Only the host is set — username/password/from are missing.
    let err = EmailConfig::from_source(source(&[("SES_SMTP_HOST", "h")])).unwrap_err();
    assert!(err.contains("SES_SMTP_USERNAME"), "error should name the first missing field, got: {err}");

    // Nothing set at all.
    assert!(EmailConfig::from_source(source(&[])).is_err());
}

#[test]
fn builds_activation_and_reset_links_from_frontend_url() {
    let cfg = EmailConfig::from_source(source(&[
        ("SES_SMTP_HOST", "h"),
        ("SES_SMTP_USERNAME", "u"),
        ("SES_SMTP_PASSWORD", "p"),
        ("SES_FROM_ADDRESS", "f@example.com"),
    ])).unwrap();
    let svc = EmailService::from_parts(cfg, "https://app.example.com".to_string());

    assert_eq!(svc.activation_link("abc123"), "https://app.example.com/activate?token=abc123");
    assert_eq!(svc.reset_link("xyz789"), "https://app.example.com/reset?token=xyz789");
}
