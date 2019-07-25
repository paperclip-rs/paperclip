//! Interior mutability stuff.

use crate::v2::models::SchemaRepr;
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

/// Wrapper over `Rc<RefCell<T>>` to offer deserialization support.
#[derive(Debug)]
pub struct RcRefCell<S>(Rc<RefCell<S>>);

impl<S> Clone for RcRefCell<S> {
    fn clone(&self) -> Self {
        RcRefCell(self.0.clone())
    }
}

impl<S> From<S> for RcRefCell<S> {
    fn from(s: S) -> Self {
        RcRefCell(Rc::new(RefCell::new(s)))
    }
}

impl<S> Deref for RcRefCell<S> {
    type Target = Rc<RefCell<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for RcRefCell<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Serialize for RcRefCell<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de, S> Deserialize<'de> for RcRefCell<S>
where
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = S::deserialize(deserializer)?;
        Ok(value.into())
    }
}

/// Wrapper over `Arc<RwLock<T>>` to offer deserialization support.
#[derive(Debug)]
pub struct ArcRwLock<S>(Arc<RwLock<S>>);

impl<S> From<S> for ArcRwLock<S> {
    fn from(s: S) -> Self {
        ArcRwLock(Arc::new(RwLock::new(s)))
    }
}

impl<S> Clone for ArcRwLock<S> {
    fn clone(&self) -> Self {
        ArcRwLock(self.0.clone())
    }
}

impl<S> Deref for ArcRwLock<S> {
    type Target = Arc<RwLock<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for ArcRwLock<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Serialize for ArcRwLock<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.read().serialize(serializer)
    }
}

impl<'de, S> Deserialize<'de> for ArcRwLock<S>
where
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = S::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl<T> Serialize for SchemaRepr<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaRepr::Raw(s) => s.serialize(serializer),
            SchemaRepr::Resolved { new, .. } => new.serialize(serializer),
        }
    }
}
