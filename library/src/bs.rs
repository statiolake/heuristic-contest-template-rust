#[derive(Debug)]
pub struct BSQueue<State, F, R> {
    width: usize,
    buffer: Vec<Option<State>>,
    key_fn: F,
    min_key: Option<R>,
    is_dirty: bool,
    pop_index: usize,
}

impl<BS, F, R> BSQueue<BS, F, R>
where
    F: Fn(&BS) -> R + Clone,
    R: Copy + Ord,
{
    pub fn new(width: usize, key_fn: F) -> Self {
        assert!(width > 0, "width must be positive integer");
        let queue = Vec::with_capacity(width * 2);

        Self {
            width,
            buffer: queue,
            key_fn,
            min_key: None,
            is_dirty: false,
            pop_index: 0,
        }
    }

    pub fn prepare_next_queue(&self) -> Self {
        Self::new(self.width, self.key_fn.clone())
    }

    pub fn push(&mut self, bs: BS) {
        if let Some(min_key) = self.min_key {
            let key = (self.key_fn)(&bs);
            if key < min_key {
                return;
            }
        }

        self.buffer.push(Some(bs));
        self.is_dirty = true;
        if self.buffer.len() == self.buffer.capacity() {
            self.sort_truncate();
        }
    }

    pub fn pop(&mut self) -> Option<BS> {
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

    pub fn sort_truncate(&mut self) {
        if !self.is_dirty {
            return;
        }

        self.buffer.drain(0..self.pop_index);
        self.pop_index = 0;

        self.buffer
            .sort_unstable_by_key(|bs| (self.key_fn)(bs.as_ref().unwrap()));
        self.buffer.reverse();
        if self.buffer.len() < self.width {
            return;
        }

        self.buffer.truncate(self.width);
        self.min_key = Some((self.key_fn)(self.buffer.last().unwrap().as_ref().unwrap()));
        self.is_dirty = false;
    }
}
