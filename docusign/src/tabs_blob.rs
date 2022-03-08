use anyhow::Result;

use crate::Client;

pub struct TabsBlob {
    pub client: Client,
}

impl TabsBlob {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TabsBlob { client }
    }

    /**
     * Gets encrypted tabs for envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/tabs_blob` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str, envelope_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/tabs_blob",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(envelope_id),
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates encrypted tabs for envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/tabs_blob` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(&self, account_id: &str, envelope_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/tabs_blob",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(envelope_id),
        );

        self.client.put(&url, None).await
    }
}
