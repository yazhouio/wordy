use std::{sync::{Mutex, Arc}, collections::HashMap};

use flume::Sender;
use uuid::Uuid;

use crate::utils::event;

pub mod state;
pub mod router;


pub type UserPeerMap = Arc<Mutex<HashMap<Arc<Uuid>, Sender<Arc<event::WsRequest>>>>>;
pub type UserUUidMap = Arc<Mutex<HashMap<u64, Vec<Arc<Uuid>>>>>;
