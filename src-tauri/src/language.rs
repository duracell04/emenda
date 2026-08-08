use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageProfile {
    #[serde(rename = "de-CH")]
    DeCh,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "ka-GE")]
    KaGe,
    #[serde(rename = "ru-RU")]
    RuRu,
}

impl LanguageProfile {
    pub const ALL: [Self; 6] = [
        Self::DeCh,
        Self::EnGb,
        Self::EnUs,
        Self::FrFr,
        Self::KaGe,
        Self::RuRu,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::DeCh => "de-CH",
            Self::EnGb => "en-GB",
            Self::EnUs => "en-US",
            Self::FrFr => "fr-FR",
            Self::KaGe => "ka-GE",
            Self::RuRu => "ru-RU",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::DeCh => "Swiss Standard German",
            Self::EnGb => "British English",
            Self::EnUs => "American English",
            Self::FrFr => "French",
            Self::KaGe => "Georgian",
            Self::RuRu => "Russian",
        }
    }

    pub const fn prompt_instruction(self) -> &'static str {
        match self {
            Self::DeCh => {
                "Use Swiss Standard German (de-CH): use ss rather than ß, prefer Swiss spelling and vocabulary where relevant, and preserve the author's formality, names, legal terms, brands, and domain vocabulary."
            }
            Self::EnGb => {
                "Use British English (en-GB), including British spelling and punctuation conventions, while preserving the author's register and terminology."
            }
            Self::EnUs => {
                "Use American English (en-US), including American spelling and punctuation conventions, while preserving the author's register and terminology."
            }
            Self::FrFr => {
                "Use standard French as used in France (fr-FR), preserving the author's register, terminology, names, and embedded foreign-language fragments."
            }
            Self::KaGe => {
                "Use modern standard Georgian (ka-GE), recognise Georgian script, and preserve the author's register, terminology, names, and embedded foreign-language fragments."
            }
            Self::RuRu => {
                "Use modern standard Russian (ru-RU), preserving the author's register, terminology, names, and embedded foreign-language fragments."
            }
        }
    }
}

impl fmt::Display for LanguageProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl FromStr for LanguageProfile {
    type Err = ParseLanguageError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "de-CH" => Ok(Self::DeCh),
            "en-GB" => Ok(Self::EnGb),
            "en-US" => Ok(Self::EnUs),
            "fr-FR" => Ok(Self::FrFr),
            "ka-GE" => Ok(Self::KaGe),
            "ru-RU" => Ok(Self::RuRu),
            _ => Err(ParseLanguageError(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageMode {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "de-CH")]
    DeCh,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "ka-GE")]
    KaGe,
    #[serde(rename = "ru-RU")]
    RuRu,
}

impl LanguageMode {
    pub const ALL: [Self; 7] = [
        Self::Auto,
        Self::DeCh,
        Self::EnGb,
        Self::EnUs,
        Self::FrFr,
        Self::KaGe,
        Self::RuRu,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::DeCh => "de-CH",
            Self::EnGb => "en-GB",
            Self::EnUs => "en-US",
            Self::FrFr => "fr-FR",
            Self::KaGe => "ka-GE",
            Self::RuRu => "ru-RU",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "Automatic",
            Self::DeCh => "Swiss Standard German",
            Self::EnGb => "British English",
            Self::EnUs => "American English",
            Self::FrFr => "French",
            Self::KaGe => "Georgian",
            Self::RuRu => "Russian",
        }
    }

    pub const fn fixed_profile(self) -> Option<LanguageProfile> {
        match self {
            Self::Auto => None,
            Self::DeCh => Some(LanguageProfile::DeCh),
            Self::EnGb => Some(LanguageProfile::EnGb),
            Self::EnUs => Some(LanguageProfile::EnUs),
            Self::FrFr => Some(LanguageProfile::FrFr),
            Self::KaGe => Some(LanguageProfile::KaGe),
            Self::RuRu => Some(LanguageProfile::RuRu),
        }
    }

    pub fn request_instruction(self) -> String {
        match self.fixed_profile() {
            Some(profile) => format!(
                "The user selected a fixed language profile. Return detectedLanguage as {}. {}",
                profile.code(),
                profile.prompt_instruction()
            ),
            None => concat!(
                "Identify the dominant supported profile in the same request. ",
                "German maps to de-CH. English defaults to en-GB, but preserve clearly ",
                "American usage as en-US. French maps to fr-FR, Georgian to ka-GE, and ",
                "Russian to ru-RU. Preserve quotations, names, and short embedded passages ",
                "in other languages."
            )
            .to_owned(),
        }
    }
}

impl fmt::Display for LanguageMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl FromStr for LanguageMode {
    type Err = ParseLanguageError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "de-CH" => Ok(Self::DeCh),
            "en-GB" => Ok(Self::EnGb),
            "en-US" => Ok(Self::EnUs),
            "fr-FR" => Ok(Self::FrFr),
            "ka-GE" => Ok(Self::KaGe),
            "ru-RU" => Ok(Self::RuRu),
            _ => Err(ParseLanguageError(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLanguageError(String);

impl fmt::Display for ParseLanguageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unsupported language mode: {}", self.0)
    }
}

impl std::error::Error for ParseLanguageError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_modes_serialize_as_public_codes() {
        assert_eq!(
            serde_json::to_string(&LanguageMode::Auto).unwrap(),
            r#""auto""#
        );
        assert_eq!(
            serde_json::to_string(&LanguageMode::KaGe).unwrap(),
            r#""ka-GE""#
        );
    }

    #[test]
    fn every_profile_round_trips_its_code() {
        for profile in LanguageProfile::ALL {
            assert_eq!(profile.code().parse::<LanguageProfile>().unwrap(), profile);
        }
    }

    #[test]
    fn automatic_mode_documents_language_defaults() {
        let instruction = LanguageMode::Auto.request_instruction();
        assert!(instruction.contains("German maps to de-CH"));
        assert!(instruction.contains("English defaults to en-GB"));
        assert!(instruction.contains("Georgian to ka-GE"));
    }
}
