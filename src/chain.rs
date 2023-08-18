use crate::esplora::EsploraSyncClient;
use crate::logger::BenLogger;
use lightning::chain::chaininterface::BroadcasterInterface;
use lightning::chain::Filter;
use std::sync::Arc;

pub struct BenChain {
    pub tx_sync: Arc<EsploraSyncClient<Arc<BenLogger>>>,
}

impl BenChain {
    pub fn new(tx_sync: Arc<EsploraSyncClient<Arc<BenLogger>>>) -> Self {
        Self { tx_sync }
    }
}

impl Filter for BenChain {
    fn register_tx(&self, txid: &bitcoin::Txid, script_pubkey: &bitcoin::Script) {
        self.tx_sync.register_tx(txid, script_pubkey)
    }

    fn register_output(&self, output: lightning::chain::WatchedOutput) {
        self.tx_sync.register_output(output)
    }
}

// For broadcasting a transaction, we should have a BDK wallet created
// The wallet impl will have the broadcast
impl BroadcasterInterface for BenChain {
    fn broadcast_transactions(&self, txs: &[&bitcoin::Transaction]) {
        todo!()
    }
}
