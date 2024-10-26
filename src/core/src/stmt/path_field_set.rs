use crate::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PathFieldSet {
    // TODO: rewrite as a bitfield set
    container: HashSet<stmt::PathStep>,
}

impl PathFieldSet {
    pub fn new() -> PathFieldSet {
        PathFieldSet::default()
    }

    pub fn from_slice<T>(fields: &[T]) -> PathFieldSet
    where
        for<'a> &'a T: Into<stmt::PathStep>,
    {
        PathFieldSet {
            container: fields.iter().map(Into::into).collect(),
        }
    }

    pub fn insert(&mut self, val: impl Into<stmt::PathStep>) {
        self.container.insert(val.into());
    }

    pub fn unset(&mut self, field: impl Into<stmt::PathStep>) {
        self.container.remove(&field.into());
    }

    pub fn contains(&self, val: impl Into<stmt::PathStep>) -> bool {
        self.container.contains(&val.into())
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = stmt::PathStep> + '_ {
        self.container.iter().map(Clone::clone)
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }
}
