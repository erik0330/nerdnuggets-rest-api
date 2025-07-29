use super::constants::FROM_EMAIL_ADDRESS;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use rand::{thread_rng, Rng};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};
use types::{
    error::{ApiError, DbError},
    EmailVerifyType,
};
use url::Url;
use uuid::Uuid;

pub fn uuid_from_str(id: &str) -> Result<Uuid, ApiError> {
    let id = Uuid::from_str(id)
        .map_err(|_| ApiError::DbError(DbError::Str("Invalid UUID format".to_string())))?;
    Ok(id)
}

pub async fn send_auth_email(
    email: String,
    passkey: String,
    message_type: EmailVerifyType,
    ses_client: &aws_sdk_sesv2::Client,
    reset_url: Option<String>,
) -> bool {
    let (subject, content) = match message_type {
        EmailVerifyType::VerifyEmail | EmailVerifyType::AddEmail => (
            format!("Request To Verify Your Email Address(nerdnuggets.com)"),
            format!(
                r#"<div style="width: 100%; padding: 10 auto;">
                    <div style="max-width: 1000px;">
                        <div style="font-size: 40px; font-weight: bold;display: flex; justify-content: center; max-width: 1600px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;padding-top: 30px;">
                            <span style="font-size: 40;">Request To Verify Your Email Address</span>
                        </div>
                        <div style="width: 100%; margin: 30px;">
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Hi</p>
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">A request to verify your email address has been detected on NOBLEBLOCKS. To proceed, please</p>
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">enter the verification code provided below:</p>
                            <p style="font-size: 20px;font-weight: bold;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;">Verification Code: {}</p>
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">For your security, this code will expire in 3 minutes and is valid for only one use.</p>
                            <br />
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Best regards</p>
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">NOBLEBLOCKS Team</p>
                            <a href="https://www.nerdnuggets.com" style="margin-top: 20px; color: #1155cc">www.nerdnuggets.com</a>
                            <p style="background: #888888; width: 100%; height: 2px;"></p>
                            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;"><i>Please do not reply to this email as it is automatically generated.</i></p>
                            <p style="background: #888888; width: 100%; height: 2px;"></p>
                        </div>
                    </div>
                </div>"#,
                passkey
            ),
        ),
        EmailVerifyType::ResetPassword => {
            let reset_link = if let Some(url) = reset_url {
                format!(
                    r#"<p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Click the link below to reset your password:</p>
                    <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">
                        <a href="{}" style="color: #1155cc; text-decoration: underline;">Reset Password</a>
                    </p>"#,
                    url
                )
            } else {
                format!(
                    r#"<p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Enter the verification code provided below:</p>
                    <p style="font-size: 20px;font-weight: bold;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;">Verification Code: {}</p>"#,
                    passkey
                )
            };

            (
                format!("Request To Reset Your NERDNUGGETS Password(nerdnuggets.com)"),
                format!(
                    r#"<div style="width: 100%; padding: 10 auto;">
                        <div style="max-width: 1000px;">
                            <div style="font-size: 40px; font-weight: bold;display: flex; justify-content: center; max-width: 1600px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;padding-top: 30px;">
                                <span style="font-size: 40;">Request To Reset Your NERDNUGGETS Password</span>
                            </div>
                            <div style="width: 100%; margin: 30px;">
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Hi</p>
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">A request to reset your password has been detected on NOBLEBLOCKS. To proceed, please</p>
                                {}
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">For your security, this link will expire in 15 minutes and is valid for only one use.</p>
                                <br />
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Best regards</p>
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">NOBLEBLOCKS Team</p>
                                <a href="https://nerdnuggets.com" style="margin-top: 20px; color: #1155cc">www.nerdnuggets.com</a>
                                <p style="background: #888888; width: 100%; height: 2px;"></p>
                                <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;"><i>Please do not reply to this email as it is automatically generated.</i></p>
                                <p style="background: #888888; width: 100%; height: 2px;"></p>
                            </div>
                        </div>
                    </div>"#,
                    reset_link
                ),
            )
        }
    };

    match ses_client
        .send_email()
        .from_email_address(FROM_EMAIL_ADDRESS)
        .destination(Destination::builder().to_addresses(email).build())
        .content(
            EmailContent::builder()
                .simple(
                    Message::builder()
                        .subject(
                            Content::builder()
                                .data(subject)
                                .charset("UTF-8")
                                .build()
                                .expect("Subject Content"),
                        )
                        .body(
                            Body::builder()
                                .html(
                                    Content::builder()
                                        .data(content)
                                        .charset("UTF-8")
                                        .build()
                                        .expect("Message Content"),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn send_email(
    email: String,
    subject: String,
    content: String,
    ses_client: &aws_sdk_sesv2::Client,
) -> bool {
    match ses_client
        .send_email()
        .from_email_address(FROM_EMAIL_ADDRESS)
        .destination(Destination::builder().to_addresses(email).build())
        .content(
            EmailContent::builder()
                .simple(
                    Message::builder()
                        .subject(
                            Content::builder()
                                .data(subject)
                                .charset("UTF-8")
                                .build()
                                .expect("Subject Content"),
                        )
                        .body(
                            Body::builder()
                                .html(
                                    Content::builder()
                                        .data(content)
                                        .charset("UTF-8")
                                        .build()
                                        .expect("Message Content"),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn generate_random_number(from: u32, to: u32) -> u32 {
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(from..=to);
    random_number
}

pub fn generate_doi_identifier(now: Option<DateTime<Utc>>, article_id: &str) -> String {
    format!(
        "{}.{}",
        now.unwrap_or(Utc::now()).format("%Y%m"),
        &article_id[2..article_id.len() - 1]
    )
}

pub fn datetime_to_string(datetime: Option<DateTime<Utc>>) -> String {
    datetime
        .map(|d| d.format("%b %d, %Y").to_string())
        .unwrap_or_default()
}

pub fn string_to_float(uuid: &str) -> f32 {
    let mut hasher = DefaultHasher::new();
    uuid.hash(&mut hasher);
    let hash_value = hasher.finish();

    // Convert the hash value to a float in the range -1.0 to 1.0
    let scaled_value = (hash_value as f64 / u64::MAX as f64) * 2.0 - 1.0;
    scaled_value as f32
}

pub fn get_base_url(input: &str) -> String {
    // Try parsing the input as a URL
    if let Ok(url) = Url::parse(input) {
        // Get the base URL by trimming the path (removes everything after the last '/')
        let mut base_url = url.clone();
        base_url.set_path(""); // Remove the path entirely
        base_url.to_string()
    } else {
        input.to_string() // Return the original string if parsing fails
    }
}

pub fn is_valid_email(email: &str) -> bool {
    if email.contains('+') {
        return false;
    }
    EmailAddress::is_valid(email)
}

pub fn get_article_id_from_proposal_id(proposal_id: &str) -> String {
    format!("NB{}A", proposal_id)
}
