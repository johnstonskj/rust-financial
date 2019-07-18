/*!
Wrapper around the external metrics crate.
*/

use std::collections::HashMap;

use log::Level;
use metrics::exporters::LogExporter;
use metrics::recorders::TextRecorder;
use metrics::{Controller, Receiver};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ApiName {
    Price,
    DelayedQuote,
    Quote,
    Intraday,
    Historical,
    Peers,
    TargetPrice,
    ConsensusEPS,
    ConsensusRatings,
}

lazy_static! {
    static ref COSTS: HashMap<ApiName, u16> = {
        let mut m = HashMap::new();
        m.insert(ApiName::Price, 1);
        m.insert(ApiName::Quote, 1);
        m.insert(ApiName::DelayedQuote, 1);
        m.insert(ApiName::Intraday, 1);
        m.insert(ApiName::Historical, 10);
        m.insert(ApiName::Peers, 500);
        m.insert(ApiName::TargetPrice, 500);
        m.insert(ApiName::ConsensusEPS, 10_000);
        m.insert(ApiName::ConsensusRatings, 1_000);
        m
    };
    static ref RECEIVER: Receiver = Receiver::builder()
        .build()
        .expect("failed to create receiver");
    static ref EXPORTER: LogExporter<Controller, TextRecorder> =
        LogExporter::new(RECEIVER.get_controller(), TextRecorder::new(), Level::Info);
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn record_api_use(api: ApiName) {
    record_api_usage(api, 1)
}

pub fn record_api_usage(api: ApiName, count: u16) {
    debug!("recording API usage for {:?} x {}", api, count);
    let cost_per_call = COSTS.get(&api).unwrap();
    RECEIVER
        .get_sink()
        .record_count("IEX::API::total_cost", (count * cost_per_call) as u64);
    RECEIVER
        .get_sink()
        .record_count(format!("IEX::API::{:?}::count", api), count as u64);
}

pub fn record_to_log() {
    debug!("committing metrics to log");
    EXPORTER.turn();
}
