mod app;
mod utils;

use std::{path::PathBuf, sync::Arc};

pub use app::TuiApp;
use tokio::sync::mpsc::Sender;
pub use utils::{init, restore};

use crate::BlenderVersion;


pub type TxMessage = Arc<Sender<Message>>;
pub enum Message {
    GetLinksResult(Vec<BlenderVersion>),

    GetVersionUpdate(String),
    GetVersionResult(PathBuf),
    ExtractResult,

    Error(String),
}
