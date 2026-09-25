//! Transactional email via Amazon SES.
//!
//! This uses SES's SMTP interface through `lettre` (already a dependency) rather than the SES API,
//! so no AWS SDK / credential chain is required — just SES SMTP credentials and a verified sender.
//! Configuration is read from the environment (see `EmailConfig::from_env`). The `EmailService`
//! centralizes the two transactional emails the app sends: account activation and password reset.

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

/// Read an env var, treating an empty/whitespace value as unset.
fn env_opt(key: &str) -> Option<String> {
    std::env::var(key).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// Connection settings for SES's SMTP endpoint. Prefers `SES_*` names and falls back to the older
/// generic `SMTP_*` names so existing configuration keeps working.
#[derive(Clone, Debug)]
pub struct EmailConfig {
    /// SES SMTP host, e.g. `email-smtp.us-east-1.amazonaws.com`.
    pub smtp_host: String,
    /// SMTP submission port. 587 (STARTTLS, default) or 465 (implicit TLS).
    pub smtp_port: u16,
    /// SES SMTP username (an IAM SMTP credential — NOT an AWS access key id).
    pub username: String,
    /// SES SMTP password (the paired SMTP credential secret).
    pub password: String,
    /// The From address, which must be a verified SES identity, e.g. `QView <no-reply@example.com>`.
    pub from_address: String,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self, String> {
        Self::from_source(|key| std::env::var(key).ok())
    }

    /// Build the config from an arbitrary key→value source (env in production; a fixture in tests).
    /// Prefers the `SES_*` names and falls back to the legacy `SMTP_*` names; blank values count as
    /// unset. `SES_SMTP_PORT` defaults to 587 (STARTTLS).
    pub fn from_source(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let get = |key: &str| lookup(key).map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
        let first = |primary: &str, fallback: &str| get(primary).or_else(|| get(fallback));

        let smtp_host = first("SES_SMTP_HOST", "SMTP_HOST")
            .ok_or("SES_SMTP_HOST (or SMTP_HOST) is not set")?;
        let username = first("SES_SMTP_USERNAME", "SMTP_USER")
            .ok_or("SES_SMTP_USERNAME (or SMTP_USER) is not set")?;
        let password = first("SES_SMTP_PASSWORD", "SMTP_PASS")
            .ok_or("SES_SMTP_PASSWORD (or SMTP_PASS) is not set")?;
        let from_address = first("SES_FROM_ADDRESS", "SMTP_FROM")
            .ok_or("SES_FROM_ADDRESS (or SMTP_FROM) is not set")?;
        let smtp_port = first("SES_SMTP_PORT", "SMTP_PORT")
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(587);
        Ok(Self { smtp_host, smtp_port, username, password, from_address })
    }
}

/// Sends the application's transactional emails through SES.
pub struct EmailService {
    config: EmailConfig,
    /// Base URL of the frontend, used to build the links embedded in emails.
    frontend_url: String,
}

impl EmailService {
    /// Build the service from the environment. Returns `Err` (rather than panicking) when SES isn't
    /// configured, so callers can degrade gracefully (e.g. log the link in development).
    pub fn from_env() -> Result<Self, String> {
        Ok(Self::from_parts(
            EmailConfig::from_env()?,
            env_opt("FRONTEND_URL").unwrap_or_else(|| "http://localhost:5173".to_string()),
        ))
    }

    /// Construct from explicit parts (used by tests; `from_env` is the production path).
    pub fn from_parts(config: EmailConfig, frontend_url: String) -> Self {
        Self { config, frontend_url }
    }

    pub fn activation_link(&self, token: &str) -> String {
        format!("{}/activate?token={}", self.frontend_url, token)
    }

    pub fn reset_link(&self, token: &str) -> String {
        format!("{}/reset?token={}", self.frontend_url, token)
    }

    /// Send a plain-text email through SES. Synchronous (lettre's blocking transport); email volume
    /// here is low (one message per registration / reset request).
    fn send(&self, to_email: &str, subject: &str, body: String) -> Result<(), String> {
        let message = Message::builder()
            .from(self.config.from_address.parse().map_err(|e| format!("invalid SES_FROM_ADDRESS: {e}"))?)
            .to(to_email.parse().map_err(|e| format!("invalid recipient address: {e}"))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .map_err(|e| format!("failed to build email: {e}"))?;

        let creds = Credentials::new(self.config.username.clone(), self.config.password.clone());
        // Port 465 uses implicit TLS; anything else (587 default, 2587, …) uses STARTTLS.
        let builder = if self.config.smtp_port == 465 {
            SmtpTransport::relay(&self.config.smtp_host)
        } else {
            SmtpTransport::starttls_relay(&self.config.smtp_host)
        }.map_err(|e| format!("failed to configure SES SMTP transport: {e}"))?;

        let mailer = builder.port(self.config.smtp_port).credentials(creds).build();
        mailer.send(&message).map_err(|e| format!("SES send failed: {e}"))?;
        Ok(())
    }

    /// Email a new user their account-activation link.
    pub fn send_activation_email(&self, to_email: &str, token: &str) -> Result<(), String> {
        let link = self.activation_link(token);
        self.send(
            to_email,
            "Activate your QView account",
            format!(
                "Welcome to QView!\n\nClick the link below to activate your account (expires in 24 hours):\n\n{link}\n\nIf you didn't create this account, you can safely ignore this email."
            ),
        )
    }

    /// Email a user their password-reset link.
    pub fn send_password_reset_email(&self, to_email: &str, token: &str) -> Result<(), String> {
        let link = self.reset_link(token);
        self.send(
            to_email,
            "QView Password Reset",
            format!(
                "We received a request to reset your QView password.\n\nClick the link below to choose a new password (expires in 24 hours):\n\n{link}\n\nIf you didn't request this, you can safely ignore this email and your password will stay the same."
            ),
        )
    }
}
