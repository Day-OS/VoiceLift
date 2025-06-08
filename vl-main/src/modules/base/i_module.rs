use busrt::async_trait;
use futures::future::BoxFuture;
use std::any::type_name;
use std::fmt::Debug;

#[async_trait]
pub trait IModule: Debug + Send + Sync {
    fn is_started(&self) -> bool;

    // Initialize a module
    async fn start(&mut self) -> anyhow::Result<()>;

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
