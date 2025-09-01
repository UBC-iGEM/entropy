use std::collections::{VecDeque, vec_deque::Iter};

pub struct RingBuffer<T: Copy> {
        buffer: VecDeque<T>,
        capacity: usize,
}

impl<T: Copy> RingBuffer<T> {
        pub fn with_capacity(capacity: usize) -> Self {
                Self {
                        buffer: VecDeque::with_capacity(capacity),
                        capacity,
                }
        }

        pub fn push(&mut self, value: T) {
                while self.buffer.len() >= self.capacity {
                        self.buffer.pop_front();
                }
                self.buffer.push_back(value);
        }

        pub fn last(&self) -> Option<&T> {
                self.buffer.back()
        }

        pub fn nth_last(&self, n: usize) -> Option<&T> {
                self.buffer.get(self.buffer.len() - n)
        }

        pub fn is_empty(&self) -> bool {
                self.buffer.is_empty()
        }

        pub fn iter(&self) -> Iter<'_, T> {
                self.buffer.iter()
        }

        pub fn len(&self) -> usize {
                self.buffer.len()
        }

        pub fn slice_end(&self, n: usize) -> Iter<'_, T> {
                self.buffer.range(self.buffer.len().saturating_sub(n)..)
        }
}
