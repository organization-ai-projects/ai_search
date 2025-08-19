use std::any::Any;
pub trait RoleBehavior {
    fn name(&self) -> &'static str;
    fn process(&self, input: &str) -> Box<dyn Any>;
}
