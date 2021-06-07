use hmac::{Hmac, Mac, NewMac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use super::Client;
use crate::transport::HttpLowLevel;

/// To interact with the Keys API.
pub struct ClientKeys<T> {
    pub(super) client: Client<T>,
}

impl<T> ClientKeys<T>
where
    T: HttpLowLevel,
{
    /// Create an API Key.
    pub async fn create(
        &self,
        actions: Vec<Actions>,
        collections: Vec<String>,
        description: impl Into<Option<String>>,
        expires_at: impl Into<Option<usize>>,
    ) -> crate::Result<ClientKeyCreate> {
        let create = Create {
            actions,
            collections,
            description: description.into(),
            expires_at: expires_at.into(),
        };

        let response = self
            .client
            .post("/keys", serde_json::to_vec(&create)?)
            .await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Retrieve (metadata about) a key.
    pub async fn retrieve(&self, n: usize) -> crate::Result<ClientKeyRetrieve> {
        let response = self.client.get(format!("/keys/{}", n).as_str()).await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Retrieve (metadata about) all keys.
    pub async fn retrieve_all(&self) -> crate::Result<ClientKeyRetrieveAll> {
        let response = self.client.get("/keys").await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Delete an API key given its ID.
    pub async fn delete(&self, n: usize) -> crate::Result<ClientKeyDelete> {
        let response = self.client.delete(format!("/keys/{}", n).as_str()).await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Generate a scoped search API key that can have embedded search parameters in them.
    pub async fn generate_scoped_search_key(
        key: impl AsRef<str>,
        filter_by: impl AsRef<str>,
        expires_at: usize,
    ) -> crate::Result<String> {
        let generate_scoped_search_key = GenerateScopedSearchKey {
            filter_by: filter_by.as_ref().to_string(),
            expires_at,
        };
        let params = serde_json::to_string(&generate_scoped_search_key)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref().as_bytes()).unwrap();
        mac.update(params.as_bytes());
        let result = mac.finalize();
        let digest = base64::encode(result.into_bytes());

        let key_prefix = &key.as_ref()[0..4];
        let raw_scoped_key = format!("{}{}{}", digest, key_prefix, params);

        Ok(base64::encode(raw_scoped_key.as_bytes()))
    }
}

/// Enum over the possible list of Actions.
/// Read more on this [here](https://typesense.org/docs/0.19.0/api/api-keys.html#sample-actions).
#[derive(Serialize, Deserialize)]
pub enum Actions {
    /// Allows only search requests.
    #[serde(rename = "documents:search")]
    DocumentsSearch,

    /// Allows fetching a single document.
    #[serde(rename = "documents:get")]
    DocumentsGet,

    /// Allow all kinds of collection related operations.
    #[serde(rename = "documents:*")]
    DocumentsAll,

    /// Allows a collection to be deleted.
    #[serde(rename = "collections:delete")]
    CollectionsDelete,

    /// Allows a collection to be created.
    #[serde(rename = "collections:create")]
    CollectionsCreate,

    /// Allow all kinds of collection related operations.
    #[serde(rename = "collections:*")]
    CollectionsAll,

    /// Allows all operations.
    #[serde(rename = "*")]
    All,
}

#[derive(Deserialize)]
pub struct ClientKeyCreate {
    pub id: usize,
    pub actions: Vec<Actions>,
    pub collections: Vec<String>,
    pub value: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ClientKeyRetrieve {
    pub actions: Vec<Actions>,
    pub collections: Vec<String>,
    pub description: String,
    pub id: usize,
    pub value_prefix: String,
}

#[derive(Deserialize)]
pub struct ClientKeyRetrieveAll {
    pub keys: Vec<ClientKeyRetrieve>,
}

#[derive(Deserialize)]
pub struct ClientKeyDelete {
    pub id: usize,
}

#[derive(Serialize)]
struct Create {
    actions: Vec<Actions>,
    collections: Vec<String>,
    description: Option<String>,
    expires_at: Option<usize>,
}

#[derive(Serialize)]
struct GenerateScopedSearchKey {
    filter_by: String,
    expires_at: usize,
}
