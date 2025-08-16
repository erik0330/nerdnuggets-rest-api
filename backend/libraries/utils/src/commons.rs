use super::constants::FROM_EMAIL_ADDRESS;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use rand::{thread_rng, Rng};
use std::{
    collections::HashSet,
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
            format!("Request To Verify Your Email Address(nerdnuggets.org)"),
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
                            <a href="https://www.nerdnuggets.org" style="margin-top: 20px; color: #1155cc">www.nerdnuggets.org</a>
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
                format!("Request To Reset Your NERDNUGGETS Password(nerdnuggets.org)"),
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
                                <a href="https://nerdnuggets.org" style="margin-top: 20px; color: #1155cc">www.nerdnuggets.org</a>
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
        Err(e) => {
            eprintln!("Error sending email: {:?}", e);
            false
        }
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

/// Generate a unique username from name or email following LinkedIn style
/// This function creates a username by:
/// 1. Using the full name if available, otherwise using the email prefix
/// 2. Converting to lowercase and removing special characters
/// 3. Adding numbers if the username already exists (like LinkedIn)
pub fn generate_username(
    name: Option<&str>,
    email: &str,
    existing_usernames: &HashSet<String>,
) -> String {
    // Extract the base username from name or email
    let base_username = if let Some(name) = name {
        // Use the full name (first + last name) like LinkedIn
        let name_parts: Vec<&str> = name.split_whitespace().collect();
        if name_parts.len() >= 2 {
            // Combine first and last name
            &format!("{}{}", name_parts[0], name_parts[1])
        } else if name_parts.len() == 1 {
            // Use just the first name if no last name
            name_parts[0]
        } else {
            ""
        }
    } else {
        // Use the email prefix (before @)
        email.split('@').next().unwrap_or("")
    };

    // Clean the username: lowercase, alphanumeric only (no dots, underscores, etc.)
    let clean_username = base_username
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    // If clean username is empty, use a default
    let base = if clean_username.is_empty() {
        "user".to_string()
    } else {
        clean_username
    };

    // Try the base username first
    if !existing_usernames.contains(&base) {
        return base;
    }

    // If base exists, try with numbers (like LinkedIn: johnsmith1, johnsmith2, etc.)
    let mut rng = thread_rng();
    for attempt in 1..=100 {
        // Try numbers 1-99 first (more common like LinkedIn)
        if attempt <= 99 {
            let candidate = format!("{}{}", base, attempt);
            if !existing_usernames.contains(&candidate) {
                return candidate;
            }
        } else {
            // If all common numbers are taken, use random numbers
            let random_num = rng.gen_range(100..999999);
            let candidate = format!("{}{}", base, random_num);
            if !existing_usernames.contains(&candidate) {
                return candidate;
            }
        }
    }

    // If all attempts fail, use timestamp-based username
    let timestamp = chrono::Utc::now().timestamp();
    format!("{}{}", base, timestamp)
}
