use std::cell::RefCell;

use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver, Sender},
    task::JoinHandle,
};

pub struct Waiter {
    jh: RefCell<Option<JoinHandle<()>>>,
    tx: RefCell<Option<Sender<()>>>,
}

impl Waiter {
    pub fn new(jh: JoinHandle<()>, tx: Sender<()>) -> Self {
        Self {
            jh: RefCell::new(Some(jh)),
            tx: RefCell::new(Some(tx)),
        }
    }

    pub fn unfold(&mut self) -> Option<(JoinHandle<()>, Sender<()>)> {
        if self.jh.borrow().is_none() || self.tx.borrow().is_none() {
            return None;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        let tx = self.tx.borrow_mut().take().unwrap();
        Some((jh, tx))
    }

    pub async fn wait(&self) {
        if self.jh.borrow().is_none() {
            return;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        if let Err(e) = jh.await {
            log::error!("Sync task terminated with error: {e}");
        }
    }

    pub async fn stop(&self) {
        if self.tx.borrow().is_none() {
            return;
        }
        let tx = self.tx.borrow_mut().take().unwrap();
        if tx.send(()).is_err() {
            log::error!("Sync shutdown attempt failed");
        }
    }
}

pub fn is_open(rx: &mut Receiver<()>) -> bool {
    match rx.try_recv() {
        Ok(_) | Err(TryRecvError::Closed) => false,
        Err(TryRecvError::Empty) => true,
    }
}

pub fn patch_pending_block(mut value: serde_json::Value) -> serde_json::Value {
    let n = value["transactions"]
        .as_array()
        .map(|vec| vec.len())
        .unwrap_or_default();
    for idx in 0..n {
        if value["transactions"][idx]["nonce"].as_str().is_none() {
            value["transactions"][idx]["nonce"] = serde_json::json!("0x0");
        }
    }
    value
}

pub fn patch_block(mut value: serde_json::Value) -> serde_json::Value {
    if value.get("block_hash").is_none() {
        value["block_hash"] = serde_json::json!("0x0");
    }

    if value.get("state_root").is_none() {
        value["state_root"] = serde_json::json!("0x0");
    }

    if value.get("block_number").is_none() {
        value["block_number"] = serde_json::json!(0);
    }

    patch_pending_block(value)
}
