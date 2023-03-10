use anyhow::{Result, Error};
use tendermint_rpc::{Client, HttpClient};
use tokio::runtime::{Builder, Runtime};

pub use tendermint_rpc::endpoint::status::Response as StatusResponse;
pub use tendermint_rpc::endpoint::block::Response as BlockResponse;

pub struct BlockingRpc {
    client: HttpClient,
    runtime: Runtime,
}

impl BlockingRpc {
    pub fn from_url(rpc_url: &str) -> Result<Self> {
        Ok(Self {
            client: HttpClient::new(rpc_url)?,
            runtime: Builder::new_current_thread().enable_all().build()?,
        })
    }

    pub fn block(&self, height: u32) -> Result<BlockResponse> {
        self.runtime.block_on(self.client.block(height)).map_err(Error::from)
    }

    pub fn status(&self) -> Result<StatusResponse> {
        self.runtime.block_on(self.client.status()).map_err(Error::from)
    }

    pub fn abci_query(&self, path: &str, msg: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self.runtime.block_on(self.client.abci_query(Some(path.to_string()), msg, None, false))?.value)
    }
}