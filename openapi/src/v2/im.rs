use serde::{Deserialize, Deserializer};

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct RcRefCell<S>(Rc<RefCell<S>>);

impl<S> Clone for RcRefCell<S> {
    fn clone(&self) -> Self {
        RcRefCell(self.0.clone())
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

impl<'de, S> Deserialize<'de> for RcRefCell<S>
where
    S: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = S::deserialize(deserializer)?;
        Ok(RcRefCell(Rc::new(RefCell::new(value))))
    }
}
