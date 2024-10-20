use std::{rc::Rc, sync::RwLock};

use crate::config::Config;

pub enum ActiveWidget {
    FileListWidget,
    RemoteWidget,
}

pub struct State {
    pub config: Config,
    pub active_widget: ActiveWidget,
}

pub type StateRef = Rc<RwLock<State>>;