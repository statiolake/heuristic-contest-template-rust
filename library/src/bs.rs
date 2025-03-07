use std::{
    fmt::Debug,
    rc::Rc,
    time::{Duration, Instant},
};

use itertools::Itertools;

use crate::algo::float_num::OrderedFloat;

#[derive(Debug)]
pub struct BeamSearch<'a, E>
where
    E: Evaluator + Debug,
    E::Action: Debug,
    E::State: Debug,
{
    evaluator: &'a E,
    queues: Vec<BSQueue<'a, E>>,
}

pub struct EvalAfterAction {
    turn: usize,
    eval: f64,
}

pub struct ActionApplied<S> {
    state: Option<S>,
    turn: usize,
    eval: Option<f64>,
}

pub trait Evaluator {
    type Action;
    type State;

    fn is_finished_state(&self, state: &BSState<Self::Action, Self::State>) -> bool;

    fn possible_actions(
        &self,
        state: &BSState<Self::Action, Self::State>,
    ) -> impl Iterator<Item = Self::Action>;

    fn evaluate(&self, state: &BSState<Self::Action, Self::State>) -> f64;

    fn evaluate_after_action(
        &self,
        state: &BSState<Self::Action, Self::State>,
        action: &Self::Action,
    ) -> Option<EvalAfterAction> {
        let _ = (state, action);
        None
    }

    fn apply_action(
        &self,
        state: &BSState<Self::Action, Self::State>,
        action: &Self::Action,
    ) -> ActionApplied<Self::State>;
}

impl<'a, E> BeamSearch<'a, E>
where
    E: Evaluator + Debug,
    E::Action: Debug + Clone,
    E::State: Debug + Clone,
{
    pub fn new(
        evaluator: &'a E,
        max_turn: usize,
        width: usize,
        init_queue: BSQueue<'a, E>,
    ) -> Self {
        if max_turn == 0 {
            panic!("max_turn must be positive integer");
        }

        let mut queues = (0..max_turn)
            .map(|_| BSQueue::new(evaluator, width))
            .collect_vec();
        queues[0] = init_queue;

        Self { evaluator, queues }
    }

    pub fn search(&mut self, duration: Duration) -> Option<Rc<BSState<E::Action, E::State>>> {
        let timer = Instant::now();
        let mut since_last_measured = 0;
        let mut check_tle = || {
            if since_last_measured >= 1000 {
                since_last_measured = 0;
                return timer.elapsed() >= duration;
            }

            since_last_measured += 1;
            true
        };

        let mut finished_items = Vec::new();

        'mainloop: while check_tle() {
            let mut next_queues = self
                .queues
                .iter()
                .map(|q| q.prepare_next_queue())
                .collect_vec();

            let mut tried = false;
            for state in self.queues.iter_mut().rev().flat_map(|q| q.to_vec()) {
                tried = true;
                if self.evaluator.is_finished_state(&state) {
                    finished_items.push(state);
                    continue;
                }

                for action in self.evaluator.possible_actions(&state) {
                    if !check_tle() {
                        break 'mainloop;
                    }

                    let (next_turn, next_state) = if let Some(EvalAfterAction { turn, eval }) =
                        self.evaluator.evaluate_after_action(&state, &action)
                    {
                        (
                            turn,
                            BSState {
                                prev_state: Some(Rc::clone(&state)),
                                action: action.clone(),
                                eval: Some(eval),
                                state: None,
                            },
                        )
                    } else {
                        let ActionApplied {
                            state: next_state,
                            turn,
                            eval: next_eval,
                        } = self.evaluator.apply_action(&state, &action);
                        (
                            turn,
                            BSState {
                                prev_state: Some(Rc::clone(&state)),
                                action: action.clone(),
                                eval: next_eval,
                                state: next_state,
                            },
                        )
                    };

                    next_queues[next_turn].push(next_state);
                }
            }

            eprintln!("---------");
            for (i, q) in next_queues.iter().enumerate() {
                eprintln!("queue #{}: {}", i, q.len());
            }

            self.queues = next_queues;

            if !tried {
                break;
            }
        }

        finished_items.into_iter().min_by_key(|s| -> OrderedFloat {
            s.eval
                .expect("eval must be set on push()")
                .try_into()
                .expect("eval should be able to be ordered")
        })
    }
}

#[derive(Debug)]
pub struct BSQueue<'a, E>
where
    E: Evaluator + Debug,
    E::Action: Debug,
    E::State: Debug,
{
    evaluator: &'a E,
    width: usize,
    #[allow(clippy::type_complexity)]
    buffer: Vec<Option<Rc<BSState<E::Action, E::State>>>>,
    min_key: Option<f64>,
    is_dirty: bool,
    pop_index: usize,
}

#[derive(Debug, Clone)]
pub struct BSState<A, S> {
    pub prev_state: Option<Rc<BSState<A, S>>>,
    pub action: A,
    pub eval: Option<f64>,
    pub state: Option<S>,
}

impl<'a, E> BSQueue<'a, E>
where
    E: Evaluator + Debug,
    E::Action: Debug,
    E::State: Debug,
{
    pub fn new(evaluator: &'a E, width: usize) -> Self {
        assert!(width > 0, "width must be positive integer");
        let queue = Vec::with_capacity(width * 2);

        Self {
            evaluator,
            width,
            buffer: queue,
            min_key: None,
            is_dirty: false,
            pop_index: 0,
        }
    }

    pub fn prepare_next_queue(&self) -> Self {
        Self::new(self.evaluator, self.width)
    }

    pub fn push(&mut self, bs: BSState<E::Action, E::State>) {
        let eval = bs.eval.unwrap_or_else(|| self.evaluator.evaluate(&bs));
        let mut bs = bs;
        bs.eval = Some(eval);
        if let Some(min_eval) = self.min_key {
            if eval < min_eval {
                return;
            }
        }

        self.buffer.push(Some(Rc::new(bs)));
        self.is_dirty = true;
        if self.buffer.len() == self.buffer.capacity() {
            self.sort_truncate();
        }
    }

    pub fn pop(&mut self) -> Option<Rc<BSState<E::Action, E::State>>> {
        if self.is_dirty {
            self.sort_truncate();
        }

        if self.pop_index >= self.buffer.len() {
            return None;
        }

        let bs = self.buffer[self.pop_index].take();
        self.pop_index += 1;

        bs
    }

    pub fn to_vec(&mut self) -> Vec<Rc<BSState<E::Action, E::State>>>
    where
        E::Action: Clone,
        E::State: Clone,
    {
        if self.is_dirty {
            self.sort_truncate();
        }

        self.buffer
            .iter()
            .skip(self.pop_index)
            .map(|bs| bs.as_ref().unwrap().clone())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len() == self.pop_index
    }

    pub fn len(&self) -> usize {
        self.buffer.len() - self.pop_index
    }
    pub fn sort_truncate(&mut self) {
        if !self.is_dirty {
            return;
        }

        self.buffer.drain(0..self.pop_index);
        self.pop_index = 0;

        self.buffer.sort_unstable_by_key(|bs| -> OrderedFloat {
            bs.as_ref()
                .expect("live buffer item must not be None")
                .eval
                .expect("eval must be set on push()")
                .try_into()
                .expect("eval should be able to be ordered")
        });
        self.buffer.reverse();
        if self.buffer.len() < self.width {
            return;
        }

        self.buffer.truncate(self.width);
        self.min_key = Some(
            self.buffer
                .last()
                .expect("buffer must not be empty")
                .as_ref()
                .expect("live buffer item must not be None")
                .eval
                .expect("eval must be set on push()"),
        );
        self.is_dirty = false;
    }
}
