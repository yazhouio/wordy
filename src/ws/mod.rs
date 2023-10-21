use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use flume::Sender;
use uuid::Uuid;

use crate::utils::event;

pub mod router;
pub mod state;

pub type UserPeerMap = Arc<Mutex<HashMap<Arc<Uuid>, Sender<Arc<event::WsRequest>>>>>;
pub type UserUUidMap = Arc<Mutex<HashMap<u64, Vec<Arc<Uuid>>>>>;
