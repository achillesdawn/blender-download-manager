use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc::Sender;
use crate::BlenderVersion;


pub type TxMessage = Arc<Sender<Message>>;
pub enum Message {
    Links(Vec<BlenderVersion>),

    VersionUpdate(String),
    VersionResult(PathBuf),
    
    ExtractResult,

    Error(String),
}
