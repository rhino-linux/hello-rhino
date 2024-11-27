use crate::Message::ToggleLaunch;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use iced::widget::{column, container, image, row, text, toggler, Container, Themer};
use iced::window::Position;
use iced::{window, Alignment, Background, Color, Element, Length, Size, Task, Theme};

const LOGO: &[u8] = include_bytes!("assets/logo.png");

struct HelloRhino {
    launch_on_start: bool,
    auto_launch: AutoLaunch,
}

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size {
            width: 1200.0,
            height: 800.0,
        },
        position: Position::Centered,
        ..Default::default()
    };
    iced::application(HelloRhino::title, HelloRhino::update, HelloRhino::view)
        .theme(|_| Theme::Dark)
        .window(window_settings)
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
        let header = header();
        container(header)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(move |theme| container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb8(19, 9, 60))),
                border: Default::default(),
                shadow: Default::default(),
            })
            .into()
    }
}

fn header<'a>() -> Element<'a, Message> {
    let rhino_logo = iced::widget::image(image::Handle::from_bytes(LOGO))
        .height(200.0)
        .width(200.0);
    let header = column![
        rhino_logo,
        row![
            text("Welcome to Rhino Linux").size(36).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            text("👋").shaping(text::Shaping::Advanced).size(36)
        ].align_y(Alignment::Center).spacing(10),
    ]
        .align_x(Alignment::Center);
    container(header)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}