use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;

use crate::{
    error::{Error, Result},
    id::CallbackId,
};

/// Store Closure while it's being used in JavaScript.
pub struct CallbackStore<T: ?Sized> {
    cbs: Rc<RefCell<HashMap<CallbackId, Closure<T>>>>,
}

impl<T> Clone for CallbackStore<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        CallbackStore {
            cbs: self.cbs.clone(),
        }
    }
}

impl<T> CallbackStore<T>
where
    T: ?Sized,
{
    pub fn new() -> CallbackStore<T> {
        CallbackStore {
            cbs: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add(&self, id: CallbackId, cb: Closure<T>) -> Result<()> {
        match self.cbs.try_borrow_mut() {
            Ok(mut cbs) => {
                cbs.insert(id, cb);
                Ok(())
            }
            Err(e) => Err(Error::Unexpected(format!("Couldn't borrow cbs: {e}"))),
        }
    }

    pub fn remove(&self, id: &CallbackId) -> Result<()> {
        match self.cbs.try_borrow_mut() {
            Ok(mut cbs) => {
                cbs.remove(id);
                Ok(())
            }
            Err(e) => Err(Error::Unexpected(format!("Couldn't borrow cbs: {e}"))),
        }
    }
}
