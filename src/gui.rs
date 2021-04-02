use iced::{Element, Sandbox, Settings, Text};

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        MyApp
    }

    fn title(&self) -> String {
        String::from("Iced")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hi, Iced").into()
    }
}

pub fn gui_main() {
    if let Err(err) = MyApp::run(Settings::default()) {
        eprintln!("Gui Error: {}", err);
    };
}
