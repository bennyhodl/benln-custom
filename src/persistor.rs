use bitcoin::hashes::hex::FromHex;
use bitcoin::BlockHash;
use bitcoin::Txid;
use lightning::chain::chainmonitor::MonitorUpdateId;
use lightning::chain::chainmonitor::Persist;
use lightning::chain::channelmonitor::ChannelMonitor;
use lightning::chain::channelmonitor::ChannelMonitorUpdate;
use lightning::chain::transaction::OutPoint;
use lightning::chain::ChannelMonitorUpdateStatus;
use lightning::io::Cursor;
use lightning::sign::WriteableEcdsaChannelSigner;
use lightning::sign::{EntropySource, SignerProvider};
use lightning::util::ser::ReadableArgs;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

pub struct BenPersistor {
    pub path_to_channel_data: String,
}

impl BenPersistor {
    /// Initialize a new FilesystemPersister and set the path to the individual channels'
    /// files.
    pub fn new(path_to_channel_data: String) -> Self {
        Self {
            path_to_channel_data,
        }
    }

    /// Get the directory which was provided when this persister was initialized.
    pub fn get_data_dir(&self) -> String {
        self.path_to_channel_data.clone()
    }

    /// Read `ChannelMonitor`s from disk.
    pub fn read_channelmonitors<ES: Deref, SP: Deref>(
        &self,
        entropy_source: ES,
        signer_provider: SP,
    ) -> std::io::Result<
        Vec<(
            BlockHash,
            ChannelMonitor<<SP::Target as SignerProvider>::Signer>,
        )>,
    >
    where
        ES::Target: EntropySource + Sized,
        SP::Target: SignerProvider + Sized,
    {
        let mut path = PathBuf::from(&self.path_to_channel_data);
        path.push("monitors");
        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }
        let mut res = Vec::new();
        for file_option in fs::read_dir(path)? {
            let file = file_option.unwrap();
            let owned_file_name = file.file_name();
            let filename = owned_file_name.to_str().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "File name is not a valid utf8 string",
                )
            })?;
            if !filename.is_ascii() || filename.len() < 65 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid ChannelMonitor file name",
                ));
            }
            if filename.ends_with(".tmp") {
                // If we were in the middle of committing an new update and crashed, it should be
                // safe to ignore the update - we should never have returned to the caller and
                // irrevocably committed to the new state in any way.
                continue;
            }

            let txid: Txid = Txid::from_hex(filename.split_at(64).0).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid tx ID in filename")
            })?;

            let index: u16 = filename.split_at(65).1.parse().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid tx index in filename",
                )
            })?;

            let contents = fs::read(&file.path())?;
            let mut buffer = Cursor::new(&contents);
            match <(
                BlockHash,
                ChannelMonitor<<SP::Target as SignerProvider>::Signer>,
            )>::read(&mut buffer, (&*entropy_source, &*signer_provider))
            {
                Ok((blockhash, channel_monitor)) => {
                    if channel_monitor.get_funding_txo().0.txid != txid
                        || channel_monitor.get_funding_txo().0.index != index
                    {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "ChannelMonitor was stored in the wrong file",
                        ));
                    }
                    res.push((blockhash, channel_monitor));
                }
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Failed to deserialize ChannelMonitor: {}", e),
                    ))
                }
            }
        }
        Ok(res)
    }
}

impl<ChannelSigner: WriteableEcdsaChannelSigner> Persist<ChannelSigner> for BenPersistor {
    fn persist_new_channel(
        &self,
        _channel_id: OutPoint,
        _data: &ChannelMonitor<ChannelSigner>,
        _update_id: MonitorUpdateId,
    ) -> ChannelMonitorUpdateStatus {
        println!("Updating persisted channel");
        ChannelMonitorUpdateStatus::Completed
    }

    fn update_persisted_channel(
        &self,
        _channel_id: OutPoint,
        _update: Option<&ChannelMonitorUpdate>,
        _data: &ChannelMonitor<ChannelSigner>,
        _update_id: MonitorUpdateId,
    ) -> ChannelMonitorUpdateStatus {
        println!("Updating persisted channel");
        ChannelMonitorUpdateStatus::Completed
    }
}
