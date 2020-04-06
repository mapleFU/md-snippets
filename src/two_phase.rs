//! This is a naive and rubbish impl of 2PC.
//! TODO: run concurrency and support timeout.
//! TODO: using rayon and crossbeam to optimize code.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::spawn;

// use rayon::prelude::*;

pub trait TwoPhaseCoordinator<Participant: TwoPhaseParticipant> {
    fn vote(&self, p: &Participant) -> RunState {
        p.run()
    }
    fn abort(&self, p: &Participant) {
        p.abort()
    }
    fn commit(&self, p: &Participant) {
        p.commit()
    }
}

pub enum CoordinatorState {
    INIT,
    WAIT,
    ABORT,
    COMMIT,
}

pub trait TwoPhaseParticipant {
    fn run(&self) -> RunState;
    fn abort(&self);
    fn commit(&self);
}

#[derive(Clone, Copy)]
pub enum RunState {
    DoNotCommit,
    Ready,
}

pub enum ParticipantState {
    INIT,
    READY,
    ABORT,
    COMMIT,
}

pub struct CoordinatorImpl<
    P: TwoPhaseParticipant + Sync + Send,
    T: TwoPhaseCoordinator<P> + Sync + Send,
> {
    participants: Vec<Arc<P>>,
    coordinator: Arc<T>,
    txn_state: HashMap<u64, CoordinatorState>,
    txn_id: u64,
}

impl<P: 'static, T: 'static> CoordinatorImpl<P, T>
where
    P: TwoPhaseParticipant + Sync + Send,
    T: TwoPhaseCoordinator<P> + Sync + Send,
{
    pub fn new(participants: Vec<P>, coordinator: T) -> Self {
        CoordinatorImpl {
            participants: participants.into_iter().map(Arc::new).collect(),
            coordinator: Arc::new(coordinator),
            txn_state: Default::default(),
            txn_id: 0,
        }
    }

    pub fn commit(&mut self) -> RunState {
        let current_txn_id = self.txn_id;
        self.txn_id += 1;
        let state = self.prepare_impl(current_txn_id);
        match state {
            RunState::DoNotCommit => {
                self.abort_impl(current_txn_id);
            }
            RunState::Ready => {
                self.commit_impl(current_txn_id);
            }
        };
        state
    }

    fn prepare_impl(&mut self, txn_id: u64) -> RunState {
        let mut prepare_ok = true;
        self.txn_state.insert(txn_id, CoordinatorState::INIT);
        let (sender, receiver) = mpsc::channel();

        for p in self.participants.iter() {
            let p = p.clone();
            let sender = sender.clone();
            let coord = self.coordinator.clone();
            spawn(move || {
                let res = coord.vote(p.borrow());
                sender.send(res).unwrap();
            });
        }

        let length = self.participants.len();
        for _ in 0..length {
            let r = receiver.recv().unwrap();
            if let RunState::DoNotCommit = r {
                prepare_ok = false;
                break;
            }
        }

        if prepare_ok {
            RunState::Ready
        } else {
            RunState::DoNotCommit
        }
    }

    fn abort_impl(&mut self, txn_id: u64) {
        let (sender, receiver) = mpsc::channel();
        for p in self.participants.iter() {
            let p = p.clone();
            let sender = sender.clone();
            let coord = self.coordinator.clone();
            spawn(move || {
                coord.abort(p.borrow());
                sender.send(()).unwrap();
            });
        }

        let length = self.participants.len();
        for _ in 0..length {
            receiver.recv().unwrap();
        }
        self.txn_state.insert(txn_id, CoordinatorState::ABORT);
    }

    fn commit_impl(&mut self, txn_id: u64) {
        let (sender, receiver) = mpsc::channel();
        for p in self.participants.iter() {
            let p = p.clone();
            let sender = sender.clone();
            let coord = self.coordinator.clone();
            spawn(move || {
                coord.commit(p.borrow());
                sender.send(()).unwrap();
            });
        }

        let length = self.participants.len();
        for _ in 0..length {
            receiver.recv().unwrap();
        }
        self.txn_state.insert(txn_id, CoordinatorState::COMMIT);
    }
}
