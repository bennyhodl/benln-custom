use crate::convert::BlockchainInfo;
use base64;
use bitcoin::consensus::encode;
use lightning::chain::chaininterface::BroadcasterInterface;
use lightning_block_sync::{http::HttpEndpoint, rpc::RpcClient};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, Arc},
};

const MIN_FEE_RATE: u32 = 253;

#[derive(PartialEq, Eq, Hash)]
pub enum FeeTarget {
    MempoolMinimum,
    Background,
    Normal,
    HighPriority,
}

pub struct Bitcoind {
    bitcoind_rpc: Arc<RpcClient>,
    host: String,
    port: u16,
    rpc_user: String,
    rpc_pass: String,
}

impl Bitcoind {
    pub(crate) async fn new(
        host: String,
        port: u16,
        rpc_user: String,
        rpc_pass: String,
    ) -> Result<Bitcoind, std::io::Error> {
        let endpoint = HttpEndpoint::for_host(host.clone()).with_port(port.clone());
        let credentials = format!("{}:{}", rpc_user.clone(), rpc_pass.clone());
        let credentials_base64 = base64::encode(credentials);

        let bitcoind_rpc = RpcClient::new(&credentials_base64, endpoint)?;

        let _test = bitcoind_rpc
            .call_method::<BlockchainInfo>("getblockchaininfo", &vec![])
            .await
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "Could not connect to Bitcoin node.",
                )
            });

        let mut fees: HashMap<FeeTarget, AtomicU32> = HashMap::new();

        fees.insert(FeeTarget::MempoolMinimum, AtomicU32::new(MIN_FEE_RATE));
        fees.insert(FeeTarget::Background, AtomicU32::new(MIN_FEE_RATE));
        fees.insert(FeeTarget::Normal, AtomicU32::new(2000));
        fees.insert(FeeTarget::HighPriority, AtomicU32::new(5000));

        let client = Self {
            bitcoind_rpc: Arc::new(bitcoind_rpc),
            host,
            port,
            rpc_user,
            rpc_pass,
        };

        Ok(client)
    }
}

impl BroadcasterInterface for Bitcoind {
    fn broadcast_transactions(&self, txs: &[&bitcoin::Transaction]) {
        for tx in txs {
            let client = Arc::clone(&self.bitcoind_rpc);
            let tx_serialized = encode::serialize(tx);
            let tx_json = serde_json::json!(tx_serialized);
        }
    }
}
