use auto_launch::AutoLaunch;
use iced::border::Radius;
use iced::widget::{button, column, container, image, row, text, toggler};
use iced::window::Position;
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};

const LOGO: &[u8] = include_bytes!("assets/logo.png");
const UBUNTU_FONT: &[u8] = include_bytes!("assets/ubuntu_regular.ttf");

const RHINO_LINUX_ANNOUNCEMENT_URL: &str = "https://blog.rhinolinux.org/";
const RHINO_LINUX_GITHUB_URL: &str = "https://github.com/rhino-linux";

const RHINO_LINUX_WIKI_URL: &str = "https://wiki.rhinolinux.org/";

const RHINO_LINUX_DISCORD_URL: &str = "https://discord.com/invite/reSvc8Ztk3";

const RHINO_LINUX_REDDIT_URL: &str = "https://www.reddit.com/r/rhinolinux/";

const WELCOME_TEXT: &str = "Welcome, to your new Operating System. Rhino Linux is an Ubuntu-based, rolling release distribution. We hope that you enjoy Rhino Linux, and all of the unique features we offer.";

const HELLO_RHINO_TEXT: &str = "Hello, Welcome to Rhino Linux!";

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
    iced::application(HelloRhino::title, HelloRhino::update, HelloRhino::view)
        .theme(|_| Theme::Dark)
        .font(UBUNTU_FONT)
        .window_size((1200.0, 700.0))
        .position(Position::Centered)
        .run_with(HelloRhino::new)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleLaunch(bool),
    OpenAnnouncement,
    OpenGithub,
    OpenReddit,
    OpenWiki,
    OpenDiscord,
}

impl HelloRhino {
    fn new() -> (Self, Task<Message>) {
        // setup config for autostart
        let exe = std::env::current_exe().unwrap();
        let app_name = "Hello Rhino";
        let args = &["--minimized"];
        let auto_launch = AutoLaunch::new(app_name, exe.to_str().unwrap(), args);

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
        match message {
            Message::ToggleLaunch(launch) => {
                self.launch_on_start = launch;
                if self.launch_on_start {
                    self.auto_launch.enable().unwrap();
                } else {
                    self.auto_launch.disable().unwrap();
                }
                Task::none()
            }
            Message::OpenGithub => {
                if let Err(e) = webbrowser::open(RHINO_LINUX_GITHUB_URL) {
                    eprintln!("Failed to open Github url: {}", e);
                };
                Task::none()
            }
            Message::OpenWiki => {
                if let Err(e) = webbrowser::open(RHINO_LINUX_WIKI_URL) {
                    eprintln!("Failed to open Wiki url:  {}", e);
                };
                Task::none()
            }
            Message::OpenAnnouncement => {
                if let Err(e) = webbrowser::open(RHINO_LINUX_ANNOUNCEMENT_URL) {
                    eprintln!("Failed to open Announcements url: {}", e);
                };
                Task::none()
            }
            Message::OpenDiscord => {
                if let Err(e) = webbrowser::open(RHINO_LINUX_DISCORD_URL) {
                    eprintln!("Failed to open Discord url: {}", e);
                };
                Task::none()
            }
            Message::OpenReddit => {
                if let Err(e) = webbrowser::open(RHINO_LINUX_REDDIT_URL) {
                    eprintln!("Failed to open Reddit url: {}", e);
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = header();
        let main_content = main_content();
        let text_col = welcome_text();
        let footer = footer(self);

        let app_col = column![header, text_col, main_content, footer].height(Length::Fill);
        container(app_col)
            .height(Length::Fill)
            .padding(2)
            .width(Length::Fill)
            .style(move |_theme| container::Style {
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
        text(HELLO_RHINO_TEXT).size(46).font(iced::Font {
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
    let welcome_text_column =
        column![text(WELCOME_TEXT).size(26).shaping(text::Shaping::Advanced)].spacing(20);
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
            .on_press(Message::OpenAnnouncement)
            .width(300.0)
            .padding(10.0)
            .style(move |_theme, status| {
                match status {
                    button::Status::Active => ACTIVE_BUTTON_STYLE,
                    button::Status::Hovered
                    | button::Status::Disabled
                    | button::Status::Pressed => HOVERED_BUTTON_STYLE,
                }
            }),
            button(text("Wiki").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .on_press(Message::OpenWiki)
            .padding(10.0)
            .width(300.0)
            .style(move |_theme, status| {
                match status {
                    button::Status::Active => ACTIVE_BUTTON_STYLE,
                    button::Status::Hovered
                    | button::Status::Disabled
                    | button::Status::Pressed => HOVERED_BUTTON_STYLE,
                }
            }),
            button(text("Github").center().size(28).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .on_press(Message::OpenGithub)
            .padding(10.0)
            .width(300.0)
            .style(move |_theme, status| {
                match status {
                    button::Status::Active => ACTIVE_BUTTON_STYLE,
                    button::Status::Hovered
                    | button::Status::Disabled
                    | button::Status::Pressed => HOVERED_BUTTON_STYLE,
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
            .on_press(Message::OpenDiscord)
            .width(300.0)
            .padding(10.0)
            .style(move |_theme, status| {
                match status {
                    button::Status::Active => ACTIVE_BUTTON_STYLE,
                    button::Status::Hovered
                    | button::Status::Disabled
                    | button::Status::Pressed => HOVERED_BUTTON_STYLE,
                }
            }),
            button(text("Reddit").size(28).center().font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),)
            .on_press(Message::OpenReddit)
            .padding(10.0)
            .width(300.0)
            .style(move |_theme, status| {
                match status {
                    button::Status::Active => ACTIVE_BUTTON_STYLE,
                    button::Status::Hovered
                    | button::Status::Disabled
                    | button::Status::Pressed => HOVERED_BUTTON_STYLE,
                }
            }),
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
        .height(Length::Fixed(250.0))
        .into()
}

fn footer(hello_rhino: &HelloRhino) -> Element<Message> {
    let footer_row = row![row![
        text("Launch at start")
            .size(26)
            .shaping(text::Shaping::Advanced),
        toggler(hello_rhino.launch_on_start)
            .size(40.0)
            .style(move |_theme, status| {
                match status {
                    toggler::Status::Active { is_toggled } => toggler::Style {
                        background: if is_toggled {
                            Color::from_rgb8(141, 123, 228)
                        } else {
                            Color::from_rgb8(50, 50, 50)
                        },
                        background_border_width: 0.0,
                        background_border_color: Default::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Default::default(),
                    },
                    toggler::Status::Hovered { is_toggled } => toggler::Style {
                        background: if is_toggled {
                            Color::from_rgb8(141, 123, 228)
                        } else {
                            Color::from_rgb8(50, 50, 50)
                        },
                        background_border_width: 0.0,
                        background_border_color: Default::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Default::default(),
                    },
                    toggler::Status::Disabled => toggler::Style {
                        background: Color::BLACK,
                        background_border_width: 0.0,
                        background_border_color: Default::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Default::default(),
                    },
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
