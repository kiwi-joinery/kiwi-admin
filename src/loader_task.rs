use std::ops::Deref;
use std::rc::Rc;

pub trait LoadingTask {
    fn set_text(&self, x: Option<String>);
}

pub type BoxedLoadingTask = Box<dyn LoadingTask>;

#[derive(Clone)]
pub struct LoadingFunction(pub Rc<dyn Fn(LoadingTaskConfig) -> BoxedLoadingTask>);

impl PartialEq for LoadingFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for LoadingFunction {
    type Target = Rc<dyn Fn(LoadingTaskConfig) -> BoxedLoadingTask>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct LoadingTaskConfig {
    delay_full_appearance: bool,
}

impl Default for LoadingTaskConfig {
    fn default() -> Self {
        LoadingTaskConfig {
            delay_full_appearance: true,
        }
    }
}

impl LoadingTaskConfig {
    pub fn delay_full_appearance(self, delay_full_appearance: bool) -> Self {
        let mut x = self;
        x.delay_full_appearance = delay_full_appearance;
        x
    }
    pub fn get_delay_full_appearance(&self) -> bool {
        self.delay_full_appearance
    }
}
