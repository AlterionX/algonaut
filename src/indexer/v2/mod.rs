use algonaut_client::{indexer::v2::Client, Headers, error::ClientError};
use algonaut_core::{Address, Round};
use algonaut_model::indexer::v2::{
    AccountInfoResponse, AccountResponse, AccountTransactionResponse, ApplicationInfoResponse,
    ApplicationResponse, AssetResponse, AssetTransactionResponse, AssetsInfoResponse,
    BalancesResponse, Block, QueryAccount, QueryAccountInfo, QueryAccountTransaction,
    QueryApplicationInfo, QueryApplications, QueryAssetTransaction, QueryAssets, QueryAssetsInfo,
    QueryBalances, QueryTransaction, TransactionInfoResponse, TransactionResponse, AccountAssetsResponse, QueryAccountAssetsInfo,
};

use crate::error::AlgonautError;

#[derive(Debug)]
pub struct Indexer {
    pub(super) client: Client,
}

impl Indexer {
    /// Build a v2 client for Algorand's indexer.
    ///
    /// Returns an error if the url has an invalid format.
    pub fn new(url: &str) -> Result<Indexer, AlgonautError> {
        Self::with_headers(url, vec![])
    }

    /// Build a v2 client for Algorand's indexer.
    /// Use this initializer when interfacing with third party services, that require custom headers.
    ///
    /// Returns an error if the url or the headers have an invalid format.
    pub fn with_headers(url: &str, headers: Headers) -> Result<Indexer, AlgonautError> {
        Ok(Indexer {
            client: Client::new(url, headers)?,
        })
    }

    /// Returns Ok if healthy
    pub async fn health(&self) -> Result<(), AlgonautError> {
        Ok(self.client.health().await?)
    }

    /// Search for accounts.
    pub async fn accounts(&self, query: &QueryAccount) -> Result<AccountResponse, AlgonautError> {
        Ok(self.client.accounts(query).await?)
    }

    /// Lookup account information.
    pub async fn account_info(
        &self,
        address: &Address,
        query: &QueryAccountInfo,
    ) -> Result<AccountInfoResponse, AlgonautError> {
        Ok(self.client.account_info(address, query).await?)
    }

    pub async fn account_assets(&self, address: &Address, query: &QueryAccountAssetsInfo) -> Result<AccountAssetsResponse, ClientError> {
        Ok(self.client.account_assets(address, query).await?)
    }

    /// Lookup account transactions.
    pub async fn account_transactions(
        &self,
        address: &Address,
        query: &QueryAccountTransaction,
    ) -> Result<AccountTransactionResponse, AlgonautError> {
        Ok(self.client.account_transactions(address, query).await?)
    }

    /// Search for applications
    pub async fn applications(
        &self,
        query: &QueryApplications,
    ) -> Result<ApplicationResponse, AlgonautError> {
        Ok(self.client.applications(query).await?)
    }

    /// Lookup application.
    pub async fn application_info(
        &self,
        id: u64,
        query: &QueryApplicationInfo,
    ) -> Result<ApplicationInfoResponse, AlgonautError> {
        Ok(self.client.application_info(id, query).await?)
    }

    /// Search for assets.
    pub async fn assets(&self, query: &QueryAssets) -> Result<AssetResponse, AlgonautError> {
        Ok(self.client.assets(query).await?)
    }

    /// Lookup asset information.
    pub async fn assets_info(
        &self,
        id: u64,
        query: &QueryAssetsInfo,
    ) -> Result<AssetsInfoResponse, AlgonautError> {
        Ok(self.client.assets_info(id, query).await?)
    }

    /// Lookup the list of accounts who hold this asset.
    pub async fn asset_balances(
        &self,
        id: u64,
        query: &QueryBalances,
    ) -> Result<BalancesResponse, AlgonautError> {
        Ok(self.client.asset_balances(id, query).await?)
    }

    /// Lookup transactions for an asset.
    pub async fn asset_transactions(
        &self,
        id: u64,
        query: &QueryAssetTransaction,
    ) -> Result<AssetTransactionResponse, AlgonautError> {
        Ok(self.client.asset_transactions(id, query).await?)
    }

    /// Lookup block.
    pub async fn block(&self, round: Round) -> Result<Block, AlgonautError> {
        Ok(self.client.block(round).await?)
    }

    /// Search for transactions.
    pub async fn transactions(
        &self,
        query: &QueryTransaction,
    ) -> Result<TransactionResponse, AlgonautError> {
        Ok(self.client.transactions(query).await?)
    }

    /// Search for transactions.
    pub async fn transaction_info(
        &self,
        id: &str,
    ) -> Result<TransactionInfoResponse, AlgonautError> {
        Ok(self.client.transaction_info(id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_with_valid_url() {
        let indexer = Indexer::new("http://example.com");
        assert!(indexer.ok().is_some());
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_create_with_empty_url() {
        Indexer::new("").unwrap();
    }
}
