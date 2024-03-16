use {async_trait::async_trait, std::io};
pub mod tcp;
pub mod ws;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect_and_send(
        &self,
        timeout: Option<u32>,
        payload: serde_json::Value,
    ) -> io::Result<String>;
}

pub trait ChooseTransport {
    fn get_transport(&self) -> Box<dyn Transport>
    where
        Self: Sized;
}
