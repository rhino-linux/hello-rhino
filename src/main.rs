use crate::Message::ToggleLaunch;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use iced::widget::toggler;
use iced::{Element, Task, Theme};

struct HelloRhino {
    launch_on_start: bool,
    auto_launch: AutoLaunch,
}

fn main() -> iced::Result {
    iced::application(HelloRhino::title, HelloRhino::update, HelloRhino::view)
        .theme(|_| Theme::Dark)
        .run_with(HelloRhino::new)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleLaunch(bool),
}

impl HelloRhino {
    fn new() -> (Self, Task<Message>) {
        // setup config for autostart
        let auto_launch = AutoLaunchBuilder::new()
            .set_app_name("hello-rhino")
            .set_app_path(std::env::current_exe().unwrap().to_str().unwrap())
            .set_use_launch_agent(true)
            .build()
            .unwrap();

        // initially set the autostart to true
        let launch_on_start = auto_launch.enable().is_ok();

        (
            Self {
                launch_on_start,
                auto_launch,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hello Rhino")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        return match message {
            Message::ToggleLaunch(launch) => {
                self.launch_on_start = launch;
                if self.launch_on_start {
                    self.auto_launch.enable().unwrap();
                } else {
                    self.auto_launch.disable().unwrap();
                }
                Task::none()
            }
        };
    }

    fn view(&self) -> Element<Message> {
        toggler(self.launch_on_start).on_toggle(ToggleLaunch).into()
    }
}
