//! Connection trait

use crate::message::Message;
use async_trait::async_trait;

/// This object contains hooks
pub struct SetupHooks {
    pub get_string: Box<dyn Fn(String) -> Result<String, ()>>,
    pub get_int: Box<dyn Fn(String) -> Result<i32, ()>>
}

#[async_trait]
pub trait Connection<E> {
    /// Runs the setup procedure for a connection.
    ///
    /// Any object that connects to some sort of network should implement this trait.
    ///
    /// # Arguments
    ///
    /// * `hooks` - A reference to a SetupHooks struct, this allows the Connection to ask for details about the connection (for example, it might ask for the IP of another node)
    ///
    fn setup(&mut self, hooks: &SetupHooks) -> Result<(), E>;

    async fn connect(&mut self);
    async fn disconnect(&mut self);

    async fn send_message(&mut self, message: Message) -> Result<(), E>;
}