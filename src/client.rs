use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://blefyi.com/api";

/// Async client for the BLEFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, BleFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(BleFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across BLE chips, profiles, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, BleFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for a BLE chip by slug.
    pub async fn chip(&self, slug: &str) -> Result<ChipDetail, BleFyiError> {
        self.get(&format!("/chip/{}/", slug)).await
    }

    /// Get details for a GATT profile by slug.
    pub async fn profile(&self, slug: &str) -> Result<ProfileDetail, BleFyiError> {
        self.get(&format!("/profile/{}/", slug)).await
    }

    /// Get details for a Bluetooth version by slug.
    pub async fn version(&self, slug: &str) -> Result<VersionDetail, BleFyiError> {
        self.get(&format!("/version/{}/", slug)).await
    }

    /// Get details for a beacon type by slug.
    pub async fn beacon(&self, slug: &str) -> Result<BeaconDetail, BleFyiError> {
        self.get(&format!("/beacon/{}/", slug)).await
    }

    /// Get details for a BLE use case by slug.
    pub async fn usecase(&self, slug: &str) -> Result<UsecaseDetail, BleFyiError> {
        self.get(&format!("/usecase/{}/", slug)).await
    }

    /// Get details for a manufacturer by slug.
    pub async fn manufacturer(&self, slug: &str) -> Result<ManufacturerDetail, BleFyiError> {
        self.get(&format!("/manufacturer/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, BleFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two BLE chips.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, BleFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random BLE chip.
    pub async fn random(&self) -> Result<ChipDetail, BleFyiError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
