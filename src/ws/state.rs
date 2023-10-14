use flume::Sender;
use uuid::Uuid;
use std::{sync::{Arc, Mutex}, collections::HashMap};

use crate::utils::event;

use super::{UserPeerMap, UserUUidMap};




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
        self.user_peer_map.lock().unwrap().insert(uuid, sender);
    }

    pub fn remove_user_peer_map(&self, uuid: Arc<Uuid>) {
        self.user_peer_map.lock().unwrap().remove(&uuid);
    }

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


#[cfg(test)]
mod tests {
    use super::*;
    use flume::unbounded;

    #[test]
    fn test_insert_user_uuid_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        state.insert_user_uuid_map(1, uuid.clone());
        let user_uuid_map = state.user_uuid_map.lock().unwrap();
        assert_eq!(user_uuid_map.get(&1), Some(&vec![uuid]));
    }

    #[test]
    fn test_insert_user_peer_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        let (sender, _) = unbounded();
        state.insert_user_peer_map(uuid.clone(), sender.clone());
        let user_peer_map = state.user_peer_map.lock().unwrap();
        assert!(user_peer_map.get(&uuid).unwrap().same_channel(&sender));
    }

    #[test]
    fn test_remove_user_peer_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        let (sender, _) = unbounded();
        state.insert_user_peer_map(uuid.clone(), sender.clone());
        state.remove_user_peer_map(uuid.clone());
        let user_peer_map = state.user_peer_map.lock().unwrap();
        assert_eq!(user_peer_map.get(&uuid).is_none(), true);
    }

    #[test]
    fn test_remove_user_uuid_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        state.insert_user_uuid_map(1, uuid.clone());
        state.remove_user_uuid_map(1, uuid.clone());
        let user_uuid_map = state.user_uuid_map.lock().unwrap();
        assert_eq!(user_uuid_map.get(&1), None);
    }

    #[test]
    fn test_get_user_uuid_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        state.insert_user_uuid_map(1, uuid.clone());
        assert_eq!(state.get_user_uuid_map(1), Some(vec![uuid]));
    }

    #[test]
    fn test_get_user_peer_map() {
        let state = WsState::new(unbounded().0);
        let uuid = Arc::new(Uuid::new_v4());
        let (sender, _) = unbounded();
        state.insert_user_peer_map(uuid.clone(), sender.clone());
        assert_eq!(state.get_user_peer_map(uuid).unwrap().same_channel(&sender), true);
    }
}