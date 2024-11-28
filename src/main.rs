use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use iced::widget::{
    button, checkbox, column, container, image, row, text, toggler, Container, Themer,
};
use iced::window::Position;
use iced::{
    window, Alignment, Background, Border, Color, Element, Length, Padding, Size, Task, Theme,
};

const LOGO: &[u8] = include_bytes!("assets/logo.png");

struct HelloRhino {
    launch_on_start: bool,
    auto_launch: AutoLaunch,
}

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size {
            width: 1200.0,
            height: 700.0,
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
        let footer = footer(&self);

        let app_col = column![header, text_col, main_content, footer].height(Length::Fill);
        container(app_col)
            .height(Length::Fill)
            .padding(5)
            .width(Length::Fill)
            .style(move |theme| container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb8(35, 30, 55))),
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
        text("Hello, welcome to Rhino Linux!")
            .size(46)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    ]
    .align_x(Alignment::Center);
    container(header)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(300.0))
        .into()
}

fn welcome_text<'a>() -> Element<'a, Message> {
    let welcome_text_column = column![
            text("Welcome, to your new Operating System. Rhino Linux is an Ubuntu-based, rolling release distribution. We hope that you enjoy Rhino Linux, and all of the unique features we offer.").size(26)
    ]
    .spacing(20);
    container(welcome_text_column)
        .padding(iced::Padding {
            top: 0.0,
            right: 20.0,
            bottom: 0.0,
            left: 20.0,
        })
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(100.0))
        .into()
}

fn main_content<'a>() -> Element<'a, Message> {
    let content_column = column![
        row![
            button(text("Announcements").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .width(300.0)
            .padding(10.0)
            .style(move |theme, status| {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(141, 123, 228))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Default::default(),
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Default::default(),
                }
            }),
            button(text("Wiki").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .padding(10.0)
            .width(300.0)
            .style(move |theme, status| {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(141, 123, 228))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Default::default(),
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Default::default(),
                }
            }),
            button(text("Github").center().size(28).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .padding(10.0)
            .width(300.0)
            .style(move |theme, status| {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(141, 123, 228))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Default::default(),
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Default::default(),
                }
            }),
        ]
        .align_y(Alignment::Center)
        .spacing(15),
        row![
            button(text("Discord").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .width(300.0)
            .padding(10.0)
            .style(move |theme, status| {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(141, 123, 228))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Default::default(),
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Default::default(),
                }
            }),
            button(text("Reddit").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .padding(10.0)
            .width(300.0)
            .style(move |theme, status| {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb8(141, 123, 228))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Default::default(),
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Default::default(),
                }
            }),
        ].align_y(Alignment::Start).padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 150.0,
        })
        .spacing(15),
    ].spacing(15);

    container(content_column)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(240.0))
        .into()
}

fn footer(hello_rhino: &HelloRhino) -> Element<Message> {
    let footer_row = row![row![
            text("Launch at start").size(26),
            toggler(hello_rhino.launch_on_start)
                .size(40.0)
                .style(move |theme, status| {
                    toggler::Style {
                        background: Color::from_rgb8(141, 123, 228),
                        background_border_width: 0.0,
                        background_border_color: Default::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Default::default(),
                    }
                })
                .on_toggle(Message::ToggleLaunch)
        ]
        .align_y(Alignment::Center)
        .spacing(5)];

    container(footer_row)
        .width(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::End)
        .height(Length::Fixed(40.0))
        .into()
}
