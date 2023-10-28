use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::utils::event;

pub mod router;
pub mod state;

pub type Uid = u64;

pub type Sender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub type UserPeerMap = Arc<Mutex<HashMap<Arc<Uuid>, Sender<Arc<event::WsRequest>>>>>;
pub type UserUUidMap = Arc<Mutex<HashMap<Uid, Vec<Arc<Uuid>>>>>;
