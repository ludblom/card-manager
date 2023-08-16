use crate::app::App;
use std::sync::Mutex;
use std::sync::Arc;


#[derive(Debug)]
pub enum IoEvent {
    GetCard(String),
}

#[derive(Clone)]
pub struct Network<'a> {
    large_search_limit: u32,
    small_search_limit: u32,
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Network {
            large_search_limit: 40,
            small_search_limit: 3,
            app,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetCard(name) => {
                self.get_card(name).await;
            }
        };
        // TODO: Wtf.. Should it not be a Future?
        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn handle_error(&mut self, e: anyhow::Error) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }

    async fn get_card(&mut self, name: String) {
        // TODO: Search for a card
    }
}
