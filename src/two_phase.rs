//! This is a naive and rubbish impl of 2PC.
//! TODO: run concurrency and support timeout.

use std::any::Any;
use std::collections::HashMap;

pub trait TwoPhaseCoordinator<Participant: TwoPhaseParticipant> {
    fn vote(&mut self, p: &Participant) -> RunState;
    fn abort(&mut self, p: &Participant);
    fn commit(&mut self, p: &Participant);
}

pub enum CoordinatorState {
    INIT,
    WAIT,
    ABORT,
    COMMIT,
}

pub trait TwoPhaseParticipant {
    fn run(&mut self) -> RunState;
    fn abort(&mut self);
    fn commit(&mut self);
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

pub struct CoordinatorImpl<P: TwoPhaseParticipant, T: TwoPhaseCoordinator<P>> {
    participants: Vec<P>,
    coordinator: T,
    txn_state: HashMap<u64, CoordinatorState>,
    txn_id: u64,
}

impl<P, T> CoordinatorImpl<P, T>
where
    P: TwoPhaseParticipant,
    T: TwoPhaseCoordinator<P>,
{
    pub fn new(participants: Vec<P>, coordinator: T) -> Self {
        CoordinatorImpl {
            participants,
            coordinator,
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
        for p in self.participants.iter_mut() {
            match p.run() {
                RunState::DoNotCommit => {
                    prepare_ok = false;
                    break;
                }
                RunState::Ready => {
                    continue;
                }
            }
        }
        if prepare_ok {
            RunState::Ready
        } else {
            RunState::DoNotCommit
        }
    }

    fn abort_impl(&mut self, txn_id: u64) {
        for p in self.participants.iter_mut() {
            p.abort();
        }
        self.txn_state.insert(txn_id, CoordinatorState::ABORT);
    }

    fn commit_impl(&mut self, txn_id: u64) {
        for p in self.participants.iter_mut() {
            p.commit();
        }
        self.txn_state.insert(txn_id, CoordinatorState::COMMIT);
    }
}
