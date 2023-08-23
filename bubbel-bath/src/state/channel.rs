use super::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock as BlockingRwLock},
};
use tokio::sync::{broadcast, Mutex};

const BROADCAST_CHANNEL_MAX: usize = 16;

#[derive(Default)]
pub struct ChannelState {
    //  Concurrent access to this state is important, so interior rwlocks are used here.
    channel_locks: BlockingRwLock<HashMap<DataChannelId, Arc<Mutex<()>>>>,
    channel_broadcasts:
        BlockingRwLock<HashMap<DataChannelId, Vec<broadcast::Sender<DataChannelResponse>>>>,
}

impl ChannelState {
    pub fn get_channel_lock(&self, id: &DataChannelId) -> Arc<Mutex<()>> {
        //  Using HashMap's entry API would be prettier, but concurrent access is crucial here.

        let channel_locks = self.channel_locks.read().unwrap();
        if !channel_locks.contains_key(id) {
            drop(channel_locks);
            let mut channel_locks = self.channel_locks.write().unwrap();
            channel_locks.insert(*id, Arc::new(Mutex::new(())));
        }
        let channel_locks = self.channel_locks.read().unwrap();
        Arc::clone(channel_locks.get(id).unwrap())
    }

    pub fn insert_listener(&self, id: &DataChannelId) -> broadcast::Receiver<DataChannelResponse> {
        let (tx, rx) = broadcast::channel(BROADCAST_CHANNEL_MAX);
        let mut channel_broadcasts = self.channel_broadcasts.write().unwrap();
        let broadcasts = channel_broadcasts.entry(*id).or_default();
        broadcasts.push(tx);
        rx
    }

    pub fn broadcast(&self, id: &DataChannelId, res: DataChannelResponse) {
        //  Using HashMap's entry API would be prettier, but concurrent access is crucial here.
        let channel_broadcasts = self.channel_broadcasts.read().unwrap();
        if !channel_broadcasts.contains_key(id) {
            drop(channel_broadcasts);
            let mut channel_broadcasts = self.channel_broadcasts.write().unwrap();
            channel_broadcasts.insert(*id, vec![]);
        }
        let channel_broadcasts = self.channel_broadcasts.read().unwrap();
        for sender in channel_broadcasts.get(id).unwrap() {
            //  TODO Sweep away bad senders.
            let _ = sender.send(res.clone());
        }
    }
}
