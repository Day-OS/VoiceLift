use futures::future::BoxFuture;
use std::any::type_name;
use std::fmt::Debug;

pub trait IModule: Debug + Send + Sync {
    fn is_started(&self) -> bool;

    // Initialize a module
    fn start(
        &mut self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn get_screen_name(&self) -> &'static str {
        let full_name = type_name::<Self>();
        match full_name.rsplit("::").next() {
            Some(name) => name,
            None => full_name,
        }
    }
    fn get_name() -> &'static str
    where
        Self: Sized,
    {
        let full_name = type_name::<Self>();
        match full_name.rsplit("::").next() {
            Some(name) => name,
            None => full_name,
        }
    }
}
