use iced::{executor, Application, Command, Element, Text};
use iced::Settings;

sturct GUI;

impl Application for GUi {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flag: ()) -> (GUI, Commnad<Self::Message>){
        (GUI, Command::none())
    }

    fn title(&self) -> String{
        String::from("DEMO")
    }
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message>{
        Command::none()
    }
    fn view(&mut self) -> Element<self::Message>{
        Text::new("Hello, world").into()
    }
}

fn main(){
    GUI::run(Settings::default());
}