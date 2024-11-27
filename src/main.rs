use crate::Message::ToggleLaunch;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use iced::widget::{column, container, image, row, text, toggler, Container, Themer, button};
use iced::window::Position;
use iced::{window, Alignment, Background, Color, Element, Length, Size, Task, Theme, Padding};

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
        decorations: true,
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
        String::from("Hello Rhino Linux")
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
        let main_content = main_content();
        let text_col = welcome_text();

        let app_col = column![
            header,
            text_col,
            main_content
        ];
        container(app_col)
            .height(Length::Fill)
            .padding(Padding::from(5))
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
            text("Welcome to Rhino Linux!").size(36).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            text("👋").shaping(text::Shaping::Advanced).size(36)
        ]
        .align_y(Alignment::Center)
        .spacing(10),
    ]
    .align_x(Alignment::Center);
    container(header)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn welcome_text<'a>() -> Element<'a, Message> {
    let welcome_text_column = column![
        row![
            text("Thank you for joining our community!").size(24).center(),text("😊").size(24).shaping(text::Shaping::Advanced)].spacing(5).align_y(Alignment::Center),
        text("We, the Rhino Linux Developers, hope that you will enjoy using Rhino Linux as much as we enjoy building it. The links below will help you get started with your new operating system. So enjoy the experience, and don’t hesitate to send us your feedback.").center().size(24)
    ].spacing(20);
    container(welcome_text_column)
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn main_content<'a>() -> Element<'a, Message> {

    let content_column = row![
        button(text("Annoucements").size(28).center()).width(300.0),
        button(text("Wiki").size(28).center()).width(300.0),
        button(text("Discord Community").center().size(28)).width(300.0),
    ].align_y(Alignment::Center).spacing(10);

    container(content_column)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()

}
