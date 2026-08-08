use crate::{error::CoreError, language::LanguageMode};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
};

pub const DEFAULT_MODEL_ID: &str = "openrouter/free";
pub const DEFAULT_HOTKEY: &str = "Ctrl+Alt+E";
pub const OPENROUTER_API_KEY_ENV: &str = "OPENROUTER_API_KEY";
const KEYRING_SERVICE: &str = "ch.zbinden.emenda";
const KEYRING_USER: &str = "openrouter-api-key";

fn default_model_id() -> String {
    DEFAULT_MODEL_ID.to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Settings {
    model_id: String,
    language_mode: LanguageMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model_id: default_model_id(),
            language_mode: LanguageMode::Auto,
        }
    }
}

impl Settings {
    pub fn new(
        model_id: impl Into<String>,
        language_mode: LanguageMode,
    ) -> Result<Self, CoreError> {
        let model_id = validated_model_id(model_id.into())?;
        Ok(Self {
            model_id,
            language_mode,
        })
    }

    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    pub const fn language_mode(&self) -> LanguageMode {
        self.language_mode
    }

    fn validate(self) -> Result<Self, CoreError> {
        Self::new(self.model_id, self.language_mode)
    }
}

/// Settings accepted from the UI. This type intentionally does not implement
/// `Serialize` or derived `Debug`, preventing accidental secret reflection.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveSettingsInput {
    #[serde(default)]
    pub api_key: ApiKeyInput,
    pub model_id: String,
    pub language_mode: LanguageMode,
}

impl SaveSettingsInput {
    pub fn validate(self) -> Result<(Settings, CredentialUpdate), CoreError> {
        let settings = Settings::new(self.model_id, self.language_mode)?;
        let credential_update = match self.api_key {
            ApiKeyInput::Unchanged => CredentialUpdate::Unchanged,
            ApiKeyInput::Remove => CredentialUpdate::Remove,
            ApiKeyInput::Replace(value) => CredentialUpdate::Replace(ApiKey::new(value)?),
        };
        Ok((settings, credential_update))
    }
}

impl fmt::Debug for SaveSettingsInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SaveSettingsInput")
            .field("api_key", &self.api_key.redacted_label())
            .field("model_id", &self.model_id)
            .field("language_mode", &self.language_mode)
            .finish()
    }
}

/// Tri-state credential input preserves the difference between an omitted
/// field (keep the saved key), JSON null (remove it), and a new key string.
#[derive(Clone, Default)]
pub enum ApiKeyInput {
    #[default]
    Unchanged,
    Remove,
    Replace(String),
}

impl ApiKeyInput {
    const fn redacted_label(&self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Remove => "remove",
            Self::Replace(_) => "[REDACTED]",
        }
    }
}

impl<'de> Deserialize<'de> for ApiKeyInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match Option::<String>::deserialize(deserializer)? {
            Some(value) => Self::Replace(value),
            None => Self::Remove,
        })
    }
}

pub enum CredentialUpdate {
    Unchanged,
    Remove,
    Replace(ApiKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSettings {
    pub api_key_configured: bool,
    pub model_id: String,
    pub language_mode: LanguageMode,
    pub hotkey: String,
}

impl PublicSettings {
    pub fn new(settings: &Settings, api_key_configured: bool) -> Self {
        Self {
            api_key_configured,
            model_id: settings.model_id.clone(),
            language_mode: settings.language_mode,
            hotkey: DEFAULT_HOTKEY.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<Settings, CoreError> {
        let json = match fs::read_to_string(&self.path) {
            Ok(json) => json,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Settings::default());
            }
            Err(error) => {
                return Err(CoreError::ConfigurationError(format!(
                    "Could not read local settings: {error}"
                )));
            }
        };

        serde_json::from_str::<Settings>(&json)
            .map_err(|error| {
                CoreError::ConfigurationError(format!("Local settings are invalid: {error}"))
            })?
            .validate()
    }

    pub fn save(&self, settings: &Settings) -> Result<(), CoreError> {
        settings.clone().validate()?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                CoreError::ConfigurationError(format!(
                    "Could not create the local settings directory: {error}"
                ))
            })?;
        }
        let json = serde_json::to_string_pretty(settings).map_err(|error| {
            CoreError::ConfigurationError(format!("Could not encode local settings: {error}"))
        })?;
        fs::write(&self.path, json).map_err(|error| {
            CoreError::ConfigurationError(format!("Could not save local settings: {error}"))
        })
    }
}

/// An API key whose debug output cannot expose the credential.
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(CoreError::ConfigurationError(
                "OpenRouter API key cannot be empty".to_owned(),
            ));
        }
        if trimmed.chars().any(char::is_whitespace) {
            return Err(CoreError::ConfigurationError(
                "OpenRouter API key cannot contain whitespace".to_owned(),
            ));
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub(crate) fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ApiKey([REDACTED])")
    }
}

pub trait CredentialStore: Send + Sync {
    fn get_api_key(&self) -> Result<Option<ApiKey>, CoreError>;
    fn set_api_key(&self, api_key: &ApiKey) -> Result<(), CoreError>;
    fn delete_api_key(&self) -> Result<(), CoreError>;
}

#[derive(Debug, Clone)]
pub struct KeyringCredentialStore {
    service: String,
    user: String,
}

impl Default for KeyringCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyringCredentialStore {
    pub fn new() -> Self {
        Self {
            service: KEYRING_SERVICE.to_owned(),
            user: KEYRING_USER.to_owned(),
        }
    }

    fn entry(&self) -> Result<keyring::Entry, CoreError> {
        keyring::Entry::new(&self.service, &self.user).map_err(|error| {
            CoreError::ConfigurationError(format!(
                "Secure credential storage is unavailable: {error}"
            ))
        })
    }
}

impl CredentialStore for KeyringCredentialStore {
    fn get_api_key(&self) -> Result<Option<ApiKey>, CoreError> {
        match self.entry()?.get_password() {
            Ok(value) => ApiKey::new(value).map(Some),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(CoreError::ConfigurationError(format!(
                "Could not read the OpenRouter key from secure storage: {error}"
            ))),
        }
    }

    fn set_api_key(&self, api_key: &ApiKey) -> Result<(), CoreError> {
        self.entry()?
            .set_password(api_key.expose_secret())
            .map_err(|error| {
                CoreError::ConfigurationError(format!(
                    "Could not save the OpenRouter key to secure storage: {error}"
                ))
            })
    }

    fn delete_api_key(&self) -> Result<(), CoreError> {
        match self.entry()?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(CoreError::ConfigurationError(format!(
                "Could not delete the OpenRouter key from secure storage: {error}"
            ))),
        }
    }
}

/// Resolve a runtime credential. An explicit user-configured OS-keyring value
/// is authoritative; the environment variable is only the development
/// fallback when secure storage has no entry.
pub fn resolve_api_key(credentials: &dyn CredentialStore) -> Result<ApiKey, CoreError> {
    if let Some(api_key) = credentials.get_api_key()? {
        return Ok(api_key);
    }

    match env::var(OPENROUTER_API_KEY_ENV) {
        Ok(value) => ApiKey::new(value),
        Err(env::VarError::NotPresent) => Err(CoreError::ConfigurationError(
            "Configure an OpenRouter API key in Settings".to_owned(),
        )),
        Err(env::VarError::NotUnicode(_)) => Err(CoreError::ConfigurationError(
            "OPENROUTER_API_KEY is not valid Unicode".to_owned(),
        )),
    }
}

pub fn api_key_available(credentials: &dyn CredentialStore) -> Result<bool, CoreError> {
    match credentials.get_api_key()? {
        Some(_) => Ok(true),
        None => Ok(env::var(OPENROUTER_API_KEY_ENV)
            .ok()
            .is_some_and(|value| !value.trim().is_empty())),
    }
}

fn validated_model_id(value: String) -> Result<String, CoreError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(CoreError::ConfigurationError(
            "OpenRouter model ID cannot be empty".to_owned(),
        ));
    }
    if value.len() > 512 || value.chars().any(char::is_control) {
        return Err(CoreError::ConfigurationError(
            "OpenRouter model ID is invalid".to_owned(),
        ));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::Mutex,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[derive(Default)]
    struct MemoryCredentialStore(Mutex<Option<String>>);

    impl CredentialStore for MemoryCredentialStore {
        fn get_api_key(&self) -> Result<Option<ApiKey>, CoreError> {
            self.0.lock().unwrap().clone().map(ApiKey::new).transpose()
        }

        fn set_api_key(&self, api_key: &ApiKey) -> Result<(), CoreError> {
            *self.0.lock().unwrap() = Some(api_key.expose_secret().to_owned());
            Ok(())
        }

        fn delete_api_key(&self) -> Result<(), CoreError> {
            *self.0.lock().unwrap() = None;
            Ok(())
        }
    }

    fn temporary_settings_path() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!(
            "emenda-settings-{}-{unique}.json",
            std::process::id()
        ))
    }

    #[test]
    fn defaults_match_the_product_contract() {
        let settings = Settings::default();
        assert_eq!(settings.model_id(), "openrouter/free");
        assert_eq!(settings.language_mode(), LanguageMode::Auto);
    }

    #[test]
    fn persisted_settings_never_contain_the_api_key() {
        let path = temporary_settings_path();
        let store = SettingsStore::new(&path);
        let settings = Settings::new("openrouter/free", LanguageMode::Auto).unwrap();
        store.save(&settings).unwrap();

        let disk = fs::read_to_string(&path).unwrap();
        assert!(!disk.contains("apiKey"));
        assert!(!disk.contains("sk-or"));
        assert_eq!(store.load().unwrap(), settings);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn public_settings_only_report_credential_presence() {
        let public = PublicSettings::new(&Settings::default(), true);
        let json = serde_json::to_value(public).unwrap();

        assert_eq!(json["apiKeyConfigured"], true);
        assert_eq!(json["modelId"], "openrouter/free");
        assert_eq!(json["languageMode"], "auto");
        assert!(json.get("apiKey").is_none());
    }

    #[test]
    fn api_key_debug_output_is_redacted() {
        let key = ApiKey::new("development-secret").unwrap();
        assert_eq!(format!("{key:?}"), "ApiKey([REDACTED])");

        let input = SaveSettingsInput {
            api_key: ApiKeyInput::Replace("development-secret".to_owned()),
            model_id: DEFAULT_MODEL_ID.to_owned(),
            language_mode: LanguageMode::Auto,
        };
        let debug = format!("{input:?}");
        assert!(!debug.contains("development-secret"));
    }

    #[test]
    fn credential_input_distinguishes_omitted_null_and_replacement() {
        let omitted: SaveSettingsInput =
            serde_json::from_str(r#"{"modelId":"openrouter/free","languageMode":"auto"}"#).unwrap();
        let removed: SaveSettingsInput = serde_json::from_str(
            r#"{"apiKey":null,"modelId":"openrouter/free","languageMode":"auto"}"#,
        )
        .unwrap();
        let replaced: SaveSettingsInput = serde_json::from_str(
            r#"{"apiKey":"new-secret","modelId":"openrouter/free","languageMode":"auto"}"#,
        )
        .unwrap();

        assert!(matches!(omitted.api_key, ApiKeyInput::Unchanged));
        assert!(matches!(removed.api_key, ApiKeyInput::Remove));
        assert!(matches!(replaced.api_key, ApiKeyInput::Replace(_)));
    }

    #[test]
    fn credential_store_supports_save_and_delete_without_exposure() {
        let store = MemoryCredentialStore::default();
        let key = ApiKey::new("stored-key").unwrap();
        store.set_api_key(&key).unwrap();
        assert!(api_key_available(&store).unwrap());
        assert_eq!(
            resolve_api_key(&store).unwrap().expose_secret(),
            "stored-key"
        );
        store.delete_api_key().unwrap();
        assert!(store.get_api_key().unwrap().is_none());
    }

    #[cfg(target_os = "windows")]
    #[test]
    #[ignore = "uses the host Windows Credential Manager"]
    fn live_windows_credential_round_trip() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let store = KeyringCredentialStore {
            service: format!("{KEYRING_SERVICE}.integration-test.{unique}"),
            user: format!("{KEYRING_USER}-{}", std::process::id()),
        };
        let key = ApiKey::new("emenda-credential-round-trip").unwrap();

        let round_trip = (|| {
            store.set_api_key(&key)?;
            let restored = store.get_api_key()?.ok_or_else(|| {
                CoreError::ConfigurationError(
                    "The test credential was not returned by Windows".to_owned(),
                )
            })?;
            if restored.expose_secret() != key.expose_secret() {
                return Err(CoreError::ConfigurationError(
                    "Windows returned a different test credential".to_owned(),
                ));
            }
            Ok(())
        })();
        let cleanup = store.delete_api_key();

        round_trip.expect("Windows Credential Manager should round-trip the isolated test value");
        cleanup.expect("the isolated Windows test credential should be removed");
        assert!(store.get_api_key().unwrap().is_none());
    }
}
