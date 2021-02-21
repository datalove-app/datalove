use iced::{executor, Application, Command, Element, Settings, Text};
use web_view::*;

fn main() {
    App::run(Settings::default())
}

struct App;

impl Application for App {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Datalove")
    }

    fn update(&mut self, _msg: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, datalove!").into()
    }
}

fn build() {
    let html = "<html><body><h1>Hello RustyWorld!</h1></body></html>";

    web_view::builder()
        .title("Datalove")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(false)
        // .frameless(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_w, _a| Ok(()))
        .run()
        .unwrap();
}
