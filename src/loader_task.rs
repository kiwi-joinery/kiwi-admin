use std::ops::Deref;
use std::rc::Rc;

pub trait LoadingTask: Drop {
    fn set_text(&self, x: Option<String>);
}

pub type BoxedLoadingTask = Box<dyn LoadingTask>;

#[derive(Clone)]
pub struct LoadingFunction(pub Rc<dyn Fn() -> BoxedLoadingTask>);

impl PartialEq for LoadingFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for LoadingFunction {
    type Target = Rc<dyn Fn() -> BoxedLoadingTask>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
