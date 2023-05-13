use iced::{Application, executor, Theme, Command, Settings};
use message::{Message, MessageType};
use route::RouteKey;
use router::Router;
use tokio;

mod widgets;
mod router;
mod route;
mod model;
mod message;
mod messenger;

struct App {
    router: Router,
    current_route: RouteKey,
}

impl App {
    fn update_route(&mut self, route: RouteKey) {
        self.current_route = route;
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (Router, RouteKey);

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App {
            router: flags.0,
            current_route: flags.1
        }, Command::none())
    }

    fn title(&self) -> String {
        "Movie player".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message.message_type() {
            MessageType::RouteChange(route) => self.update_route(route),
            _ => {
                self.router.get_mut(&self.current_route).update(message);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.router.get(&self.current_route).1.view()
    }
}

fn router() -> Router {
    Router::new()
        .add(RouteKey::Home, || Box::new(route::Home::default()))
}

#[tokio::main]
async fn main() {
    let mut settings = Settings::default();
    settings.flags = (router(), RouteKey::Home);
    
    App::run(settings).unwrap();
}
