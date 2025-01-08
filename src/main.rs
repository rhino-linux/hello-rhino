use auto_launch::AutoLaunch;
use iced::border::Radius;
use iced::widget::{button, column, container, image, row, text, toggler};
use iced::window::{Position, Settings};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Size, Task, Theme};
use tr::{tr, tr_init};

const LOGO: &[u8] = include_bytes!("assets/logo.png");
const UBUNTU_FONT: &[u8] = include_bytes!("assets/ubuntu_regular.ttf");

const ACTIVE_BUTTON_STYLE: button::Style = button::Style {
    background: Some(Background::Color(Color::from_rgba(0.55, 0.48, 0.89, 1.0))),
    text_color: Color::WHITE,
    border: Border {
        color: Color::TRANSPARENT,
        width: 0.0,
        radius: Radius {
            top_left: 10.0,
            top_right: 10.0,
            bottom_right: 10.0,
            bottom_left: 10.0,
        },
    },
    shadow: iced::Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
        offset: iced::Vector { x: 0.0, y: 0.0 },
        blur_radius: 0.0,
    },
};

const HOVERED_BUTTON_STYLE: button::Style = button::Style {
    background: Some(Background::Color(Color::from_rgba(0.47, 0.40, 0.81, 1.0))),
    text_color: Color::WHITE,
    border: Border {
        color: Color::TRANSPARENT,
        width: 0.0,
        radius: Radius {
            top_left: 10.0,
            top_right: 10.0,
            bottom_right: 10.0,
            bottom_left: 10.0,
        },
    },
    shadow: iced::Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
        offset: iced::Vector { x: 0.0, y: 0.0 },
        blur_radius: 0.0,
    },
};

struct HelloRhino {
    launch_on_start: bool,
    auto_launch: AutoLaunch,
}

fn main() -> iced::Result {
    tr_init!("/usr/share/locale/");
    let size = Size::new(700.0, 680.0);
    let window_settings = Settings {
        size,
        min_size: Some(size),
        ..Settings::default()
    };

    iced::application(HelloRhino::title, HelloRhino::update, HelloRhino::view)
        .theme(|_| Theme::Dark)
        .font(UBUNTU_FONT)
        .window(window_settings)
        .position(Position::Centered)
        .run_with(HelloRhino::new)
}

#[derive(Debug, Clone)]
enum Message {
    ToggleLaunch(bool),
    Open(String),
}

impl HelloRhino {
    fn new() -> (Self, Task<Message>) {
        let exe = std::env::current_exe().unwrap();

        let auto_launch = AutoLaunch::new("hello-rhino", exe.to_str().unwrap(), &["--minimized"]);

        // do not use auto_launch function in debug mode
        #[allow(unused_assignments)]
        let launch_on_start = if cfg!(debug_assertions) {
            auto_launch.disable().is_ok()
        } else {
            auto_launch.is_enabled().unwrap_or(false)
        };

        (
            Self {
                launch_on_start,
                auto_launch,
            },
            Task::none(),
        )
    }

    #[allow(clippy::unused_self)]
    fn title(&self) -> String {
        String::from("Hello Rhino Linux")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleLaunch(launch) => {
                self.launch_on_start = launch;
                if self.launch_on_start {
                    self.auto_launch.enable().unwrap();
                } else {
                    self.auto_launch.disable().unwrap();
                }
            }
            Message::Open(url) => {
                if let Err(e) = open::that(url) {
                    eprintln!("Failed to open url: {e}");
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let header = header();
        let main_content = main_content();
        let text_col = welcome_text();
        let footer = footer(self);

        let app_col = column![header, text_col, main_content, footer].height(Length::Fill);
        container(app_col)
            .height(Length::Fill)
            .padding(iced::Padding {
                top: 50.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            })
            .width(Length::Fill)
            .style(move |_theme| container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb8(36, 31, 49))),
                border: Border::default(),
                shadow: Shadow::default(),
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
        text(tr!("Hello, Welcome to Rhino Linux!"))
            .size(28)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    ]
    .align_x(Alignment::Center);
    container(header)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(250.0))
        .into()
}

fn welcome_text<'a>() -> Element<'a, Message> {
    let welcome_text_column = column![
        text(tr!("Welcome, to your new Operating System. Rhino Linux is an Ubuntu-based, rolling release distribution."))
        .size(20)
        .shaping(text::Shaping::Advanced),
        text(tr!("We hope that you enjoy Rhino Linux, and all of the unique features we offer."))
        .size(20)
        .shaping(text::Shaping::Advanced)
    ]
    .align_x(Alignment::Center);

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
        .height(Length::Fixed(120.0))
        .into()
}

fn create_button<'a>(label: &str, url: &str) -> Element<'a, Message> {
    button(
        text(tr!(label)).size(20).center().font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }),
    )
    .on_press(Message::Open(url.to_string()))
    .width(200.0)
    .padding(10.0)
    .style(move |_theme, status| match status {
        button::Status::Active => ACTIVE_BUTTON_STYLE,
        button::Status::Hovered | button::Status::Disabled | button::Status::Pressed => HOVERED_BUTTON_STYLE,
    })
    .into()
}

fn main_content<'a>() -> Element<'a, Message> {
    let content_column = column![
        row![
            create_button("Announcements", "https://blog.rhinolinux.org/"),
            create_button("Wiki", "https://wiki.rhinolinux.org/"),
            create_button("Github", "https://github.com/rhino-linux/"),
        ]
        .align_y(Alignment::Center)
        .spacing(15),
        row![
            create_button("Discord", "https://discord.com/invite/reSvc8Ztk3"),
            create_button("Reddit", "https://reddit.com/r/rhinolinux/"),
        ]
        .align_y(Alignment::Start)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 150.0,
        })
        .spacing(15),
    ]
    .spacing(15);

    container(content_column)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(200.0))
        .into()
}

fn footer(hello_rhino: &HelloRhino) -> Element<Message> {
    let footer_row = row![
        row![
            text(tr!("Launch at start"))
                .size(18)
                .shaping(text::Shaping::Advanced),
            toggler(hello_rhino.launch_on_start)
                .size(28.0)
                .style(move |_theme, status| match status {
                    toggler::Status::Active { is_toggled }
                    | toggler::Status::Hovered { is_toggled } => toggler::Style {
                        background: if is_toggled {
                            Color::from_rgb8(141, 123, 228)
                        } else {
                            Color::from_rgb8(50, 50, 50)
                        },
                        background_border_width: 0.0,
                        background_border_color: Color::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Color::default(),
                    },
                    toggler::Status::Disabled => toggler::Style {
                        background: Color::BLACK,
                        background_border_width: 0.0,
                        background_border_color: Color::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Color::default(),
                    },
                })
                .on_toggle(Message::ToggleLaunch),
        ]
        .align_y(Alignment::Center)
        .spacing(5),
    ];

    container(footer_row)
        .width(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::End)
        .height(Length::Fixed(25.0))
        .into()
}
