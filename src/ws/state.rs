use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use super::{UserPeerMap, UserUUidMap};
use crate::utils::event;

type Sender<T> = tokio::sync::mpsc::UnboundedSender<T>;

pub struct WsState {
    pub sender: Sender<event::ChannelMessage>,
    pub user_peer_map: UserPeerMap,
    pub user_uuid_map: UserUUidMap,
}

impl WsState {
    pub fn new(sender: Sender<event::ChannelMessage>) -> Self {
        Self {
            sender,
            user_peer_map: Arc::new(Mutex::new(HashMap::new())),
            user_uuid_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_user_uuid_map(&self, uid: u64, uuid: Arc<Uuid>) {
        let mut user_uuid_map = self.user_uuid_map.lock().unwrap();
        if let Some(uuids) = user_uuid_map.get_mut(&uid) {
            uuids.push(uuid);
        } else {
            user_uuid_map.insert(uid, vec![uuid]);
        }
    }

    pub fn insert_user_peer_map(&self, uuid: Arc<Uuid>, sender: Sender<Arc<event::WsRequest>>) {
        let mut map = self
            .user_peer_map
            .lock()
            .expect("lock user_peer_map failed");
        map.insert(uuid, sender);
    }

    #[allow(dead_code)]
    pub fn remove_user_peer_map(&self, uuid: Arc<Uuid>) {
        self.user_peer_map.lock().unwrap().remove(&uuid);
    }

    #[allow(dead_code)]
    pub fn remove_user_uuid_map(&self, uid: u64, uuid: Arc<Uuid>) {
        let mut user_uuid_map = self.user_uuid_map.lock().unwrap();
        if let Some(uuids) = user_uuid_map.get_mut(&uid) {
            uuids.retain(|x| x != &uuid);
        }
    }

    pub fn get_user_uuid_map(&self, uid: u64) -> Option<Vec<Arc<Uuid>>> {
        self.user_uuid_map.lock().unwrap().get(&uid).cloned()
    }

    pub fn get_user_peer_map(&self, uuid: Arc<Uuid>) -> Option<Sender<Arc<event::WsRequest>>> {
        self.user_peer_map.lock().unwrap().get(&uuid).cloned()
    }
}
