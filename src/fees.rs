use lightning::chain::chaininterface::{ConfirmationTarget, FeeEstimator};
use reqwest::Error;
use serde::{Deserialize, Serialize};
pub struct BenFees;

impl BenFees {
    pub fn new() -> Self {
        Self {}
    }
}

impl FeeEstimator for BenFees {
    fn get_est_sat_per_1000_weight(
        &self,
        confirmation_target: lightning::chain::chaininterface::ConfirmationTarget,
    ) -> u32 {
        let fallback = fallback_fees();

        match get_mempool_fees() {
            Err(_) => match confirmation_target {
                ConfirmationTarget::Background => fallback.background,
                ConfirmationTarget::HighPriority => fallback.high,
                ConfirmationTarget::MempoolMinimum => fallback.min,
                ConfirmationTarget::Normal => fallback.normal,
            },
            Ok(fees) => match confirmation_target {
                ConfirmationTarget::Background => fees.background,
                ConfirmationTarget::HighPriority => fees.high,
                ConfirmationTarget::MempoolMinimum => fees.min,
                ConfirmationTarget::Normal => fees.normal,
            },
        }
    }
}

struct MempoolFeeRates {
    high: u32,
    normal: u32,
    background: u32,
    min: u32,
}

#[derive(Serialize, Deserialize)]
struct MempoolFeesResponse {
    #[serde(rename = "fastestFee")]
    fastest_fee: u32,
    #[serde(rename = "halfHourFee")]
    half_hour_fee: u32,
    #[serde(rename = "hourFee")]
    hour_fee: u32,
    #[serde(rename = "economyFee")]
    economy_fee: u32,
    #[serde(rename = "minimumFee")]
    minimum_fee: u32,
}

fn get_mempool_fees() -> Result<MempoolFeeRates, Error> {
    let mempool_rates = reqwest::blocking::get("https://mempool.space/api/v1/fees/recommended")?
        .json::<MempoolFeesResponse>()
        .map_err(|e| Error::from(e))?;

    Ok(MempoolFeeRates {
        high: mempool_rates.fastest_fee * 250,
        normal: mempool_rates.economy_fee * 250,
        background: mempool_rates.hour_fee * 250,
        min: mempool_rates.minimum_fee * 250,
    })
}

fn fallback_fees() -> MempoolFeeRates {
    MempoolFeeRates {
        high: 10,
        normal: 5,
        background: 1,
        min: 1,
    }
}
