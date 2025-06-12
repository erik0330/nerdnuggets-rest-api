use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Speech {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub speech_type: i16,
    pub voice_type: i16,
    pub audio_url: String,
    pub cost: f32,
    pub status: i16,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct SpeechWithTitle {
    #[sqlx(flatten)]
    pub speech: Speech,
    pub post_title: String,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct SpeechInfo {
    pub id: Uuid,
    pub post_id: Uuid,
    pub post_title: String,
    pub user_id: Uuid,
    pub speech_type: SpeechType,
    pub voice_type: VoiceType,
    pub audio_url: String,
    pub cost: f32,
    pub status: i16,
    pub created_at: Option<DateTime<Utc>>,
}

impl SpeechWithTitle {
    pub fn to_info(&self) -> SpeechInfo {
        SpeechInfo {
            id: self.speech.id,
            post_id: self.speech.post_id,
            post_title: self.post_title.clone(),
            user_id: self.speech.user_id,
            speech_type: self.speech.speech_type.try_into().unwrap(),
            voice_type: self.speech.voice_type.try_into().unwrap(),
            audio_url: self.speech.audio_url.clone(),
            cost: self.speech.cost,
            status: self.speech.status,
            created_at: self.speech.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum VoiceType {
    #[default]
    Alloy,
    Ash,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
}

impl std::fmt::Display for VoiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_str = match self {
            VoiceType::Alloy => "alloy",
            VoiceType::Ash => "ash",
            VoiceType::Coral => "coral",
            VoiceType::Echo => "echo",
            VoiceType::Fable => "fable",
            VoiceType::Onyx => "onyx",
            VoiceType::Nova => "nova",
            VoiceType::Sage => "sage",
            VoiceType::Shimmer => "shimmer",
        };
        write!(f, "{}", variant_str)
    }
}

impl TryFrom<i16> for VoiceType {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VoiceType::Alloy),
            1 => Ok(VoiceType::Ash),
            2 => Ok(VoiceType::Coral),
            3 => Ok(VoiceType::Echo),
            4 => Ok(VoiceType::Fable),
            5 => Ok(VoiceType::Onyx),
            6 => Ok(VoiceType::Nova),
            7 => Ok(VoiceType::Sage),
            8 => Ok(VoiceType::Shimmer),
            _ => Err(format!("Invalid value for VoiceType: {}", value)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum SpeechType {
    #[default]
    ChildSummary,
    CollegeSummary,
    PhDSummary,
    ErrorSummary,
}

impl TryFrom<i16> for SpeechType {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SpeechType::ChildSummary),
            1 => Ok(SpeechType::CollegeSummary),
            2 => Ok(SpeechType::PhDSummary),
            3 => Ok(SpeechType::ErrorSummary),
            _ => Err(format!("Invalid value for SpeechType: {}", value)),
        }
    }
}
