use std::iter;
use std::ptr::NonNull;
type Link<T> = Option<NonNull<SkipNode<T>>>;

pub struct SkipNode<V> {
    pub item: Option<V>,
    pub height: usize,
    pub prev: Link<V>,
    pub links: Vec<Link<V>>,
    pub links_len: Vec<usize>,
}

impl<V> SkipNode<V> {
    pub fn head(total_levels: usize) -> Self {
        SkipNode {
            item: None,
            height: total_levels - 1,
            prev: None,
            links: iter::repeat(None).take(total_levels).collect(),
            links_len: iter::repeat(0).take(total_levels).collect(),
        }
    }

    pub fn new(item: V, height: usize) -> Self {
        Self {
            item: Some(item),
            height,
            prev: None,
            links: iter::repeat(None).take(height + 1).collect(),
            links_len: iter::repeat(0).take(height + 1).collect(),
        }
    }

    pub fn insert_at(&mut self, new_node: Box<Self>, index: usize) -> Result<&mut Self, Box<Self>> {
        assert!(self.prev.is_none(), "Only the head may insert nodes");
        assert!(
            self.height >= new_node.height,
            "You may not insert nodes with level higher than the head!"
        );
        let inserter = IndexInserter::new(index, new_node);
        inserter.act(self)
    }

    pub fn advance_at_level_mut(
        &mut self,
        level: usize,
        mut max_distance: usize,
    ) -> (&mut Self, usize) {
        self.advance_while_at_level_mut(level, move |current_node, _| {
            let travelled = current_node.links_len[level];
            if travelled <= max_distance {
                max_distance -= travelled;
                true
            } else {
                false
            }
        })
    }

    pub fn advance_while_at_level_mut(
        &mut self,
        level: usize,
        mut pred: impl FnMut(&Self, &Self) -> bool,
    ) -> (&mut Self, usize) {
        let mut current = self;
        let mut travelled = 0;
        loop {
            match current.next_if_at_level_mut(level, &mut pred) {
                Ok((node, steps)) => {
                    current = node;
                    travelled += steps;
                }
                Err(node) => return (node, travelled),
            }
        }
    }

    pub fn next_if_at_level_mut(
        &mut self,
        level: usize,
        predicate: impl FnOnce(&Self, &Self) -> bool,
    ) -> Result<(&mut Self, usize), &mut Self> {
        let next = unsafe { self.links[level].and_then(|p| p.as_ptr().as_mut()) };
        match next {
            Some(next) if predicate(self, next) => Ok((next, self.links_len[level])),
            _ => Err(self),
        }
    }
}

struct IndexInserter<V> {
    index_seek: DistanceSeeker,
    new_node: Box<SkipNode<V>>,
}

struct DistanceSeeker(usize);

impl DistanceSeeker {
    fn seek<V>(
        &mut self,
        node: &mut SkipNode<V>,
        level: usize,
    ) -> Option<(&mut SkipNode<V>, usize)> {
        let (node, distance) = node.advance_at_level_mut(level, self.0);
        if level == 0 && distance != self.0 {
            None
        } else {
            self.0 -= distance;
            Some((node, distance))
        }
    }
}

pub trait SkipListAction<T>: Sized {}
