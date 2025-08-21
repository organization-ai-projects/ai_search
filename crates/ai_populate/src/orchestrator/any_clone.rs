//! Trait utilitaire pour permettre le clonage de Box dynamiques

use std::any::Any;

pub trait AnyClone: Any + Send {
    fn clone_box(&self) -> Box<dyn AnyClone>;
}

impl<T> AnyClone for T
where
    T: 'static + Clone + Send + Any,
{
    fn clone_box(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AnyClone> {
    fn clone(&self) -> Box<dyn AnyClone> {
        self.as_ref().clone_box()
    }
}
