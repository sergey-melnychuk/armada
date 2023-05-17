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
