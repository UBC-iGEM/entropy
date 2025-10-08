use std::collections::{VecDeque, vec_deque::Iter};

/// A fixed-size circular buffer
pub struct RingBuffer<T: Copy> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T: Copy> RingBuffer<T> {
    /// Initializes a [`RingBuffer`] with the given fixed capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes a new value to the buffer
    ///
    /// If the buffer is at capacity, the oldest element is popped
    pub fn push(&mut self, value: T) {
        while self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    /// Provides a reference to the last value, or `None` if the buffer is empty
    pub fn last(&self) -> Option<&T> {
        self.buffer.back()
    }

    /// Provides a reference to the `nth` last element, or `None` if no such value exists
    pub fn nth_last(&self, n: usize) -> Option<&T> {
        self.buffer.get(self.buffer.len() - n)
    }

    /// Returns `true` if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Iterates over elements of the buffer
    pub fn iter(&self) -> Iter<'_, T> {
        self.buffer.iter()
    }

    /// Returns the number of elements currently in the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns a slice covering the last `n` elements of the buffer
    pub fn slice_end(&self, n: usize) -> (Iter<'_, T>, usize) {
        let slice = self.buffer.range(self.buffer.len().saturating_sub(n)..);
        let slice_size = slice.len();
        (slice, slice_size)
    }
}
