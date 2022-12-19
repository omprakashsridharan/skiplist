use std::cmp;

use crate::level_generator::{GeometricalLevelGenerator, LevelGenerator};
use crate::skipnode::SkipNode;

pub struct SkipList<T> {
    head: Box<SkipNode<T>>,
    len: usize,
    level_generator: GeometricalLevelGenerator,
}

impl<T> SkipList<T> {
    #[inline]
    pub fn new() -> Self {
        let lg = GeometricalLevelGenerator::new(16, 0.5);
        Self {
            head: Box::new(SkipNode::head(lg.total())),
            len: 0,
            level_generator: lg,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let levels = cmp::max(1, (capacity as f64).log2().floor() as usize);
        let lg = GeometricalLevelGenerator::new(levels, 0.5);
        Self {
            head: Box::new(SkipNode::head(lg.total())),
            len: 0,
            level_generator: lg,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        *self.head = SkipNode::head(self.level_generator.total())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index > self.len() {
            panic!("Index out of bounds.");
        }
        self.len += 1;
        let new_node = Box::new(SkipNode::new(value, self.level_generator.random()));

    }
}
