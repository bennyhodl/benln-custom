mod bitcoind;
mod chain;
mod convert;
mod esplora;
mod fees;
mod keys;
mod logger;
mod persistor;
use crate::bitcoind::Bitcoind;
use fees::BenFees;
use keys::ben_keys;
use lightning::chain::chaininterface::BroadcasterInterface;
use lightning::chain::chainmonitor;
use lightning::chain::Filter;
use lightning::sign::InMemorySigner;
use lightning::sign::KeysManager;
use lightning::util::config::UserConfig;
use logger::BenLogger;
use persistor::BenPersistor;
use std::sync::Arc;
use std::time::SystemTime;

type ChainMonitor = chainmonitor::ChainMonitor<
    InMemorySigner,
    Arc<dyn Filter + Send + Sync>,
    Arc<dyn BroadcasterInterface + Send + Sync>,
    Arc<BenFees>,
    Arc<BenLogger>,
    Arc<BenPersistor>,
>;

fn main() {
    println!("Hello, ben!");

    let ldk_data_dir = "/Users/ben/github.com/benln/.ldk";
    // Get ben keys
    let keys_seed = ben_keys(ldk_data_dir);

    // Time for entropy
    let cur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let keys_manager = Arc::new(KeysManager::new(
        &keys_seed,
        cur.as_secs(),
        cur.subsec_nanos(),
    ));

    let bitcoind = Bitcoind::new(
        String::from("localhost"),
        8332,
        String::from("rpcuser"),
        String::from("rpcpass"),
    );

    let persistor = Arc::new(BenPersistor::new(format!(
        "{}/channel_data",
        ldk_data_dir.clone()
    )));

    // Need a node struct!

    // let logger = Arc::new(BenLogger::new());
    // let esplora_url = "https://mempool.space";
    // let esplora_async = Builder::new(esplora_url).build_async();

    // let esplora_client = Arc::new(
    //     EsploraSyncClient::from_client(esplora_async.clone(), logger.clone()),
    //     logger.clone(),
    // );
    // let fee_estimator = Arc::new(BenFees::new());
    // let broadcaster = Arc::new(BenChain::new(esplora_client));

    // let chain_monitor = ChainMonitor::new(
    //     None,
    //     broadcaster.clone(),
    //     logger.clone(),
    //     fee_estimator.clone(),
    //     persistor.clone(),
    // );

    // let mut channel_monitors = persistor
    //     .read_channelmonitors(keys_manager.clone(), keys_manager.clone())
    //     .unwrap();

    let config = UserConfig::default();
}
