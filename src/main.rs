use auto_launch::AutoLaunch;
use iced::border::Radius;
use iced::widget::{button, column, container, image, row, text, toggler};
use iced::window::Position;
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Shadow, Task, Theme};
use tr::*;

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
impl Message {
    fn open(&self) -> &str {
        match self {
            Self::OpenAnnouncement => "https://blog.rhinolinux.org/",
            Self::OpenGithub => "https://github.com/rhino-linux/",
            Self::OpenReddit => "https://reddit.com/r/rhinolinux/",
            Self::OpenWiki => "https://wiki.rhinolinux.org/",
            Self::OpenDiscord => "https://discord.com/invite/reSvc8Ztk3",
            Self::ToggleLaunch(_) => panic!("Called `open` on unopenable match"),
        }
    }
}
impl HelloRhino {
    fn new() -> (Self, Task<Message>) {
        // setup config for autostart
        let exe = std::env::current_exe().unwrap();

        let auto_launch = AutoLaunch::new("Hello Rhino", exe.to_str().unwrap(), &["--minimized"]);

        // initially set the autostart to true
        #[allow(unused_assignments)]
        let mut launch_on_start = true;
        // Disable autostarting when debug running
        if cfg!(debug_assertions) {
            launch_on_start = auto_launch.disable().is_ok();
        } else {
            launch_on_start = auto_launch.enable().is_ok();
        }

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
        if let Message::ToggleLaunch(launch) = message {
            self.launch_on_start = launch;
            if self.launch_on_start {
                self.auto_launch.enable().unwrap();
            } else {
                self.auto_launch.disable().unwrap();
            }
            Task::none()
        } else {
            if let Err(e) = open::that(message.open()) {
                eprintln!("Failed to open url: {e}");
            };
            Task::none()
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
    let welcome_text_column =
        column![text(tr!("Welcome, to your new Operating System. Rhino Linux is an Ubuntu-based, rolling release distribution. We hope that you enjoy Rhino Linux, and all of the unique features we offer.")).size(26).shaping(text::Shaping::Advanced)].spacing(20);
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
            button(
                text(tr!("Announcements"))
                    .size(28)
                    .center()
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
            )
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
            button(text(tr!("Wiki")).size(28).center().font(iced::Font {
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
            button(text(tr!("Github")).center().size(28).font(iced::Font {
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
            button(text(tr!("Discord")).size(28).center().font(iced::Font {
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
            button(text(tr!("Reddit")).size(28).center().font(iced::Font {
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
        text(tr!("Launch at start"))
            .size(26)
            .shaping(text::Shaping::Advanced),
        toggler(hello_rhino.launch_on_start)
            .size(40.0)
            .style(move |_theme, status| {
                match status {
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
                        foreground_border_color: Default::default(),
                    },
                    toggler::Status::Disabled => toggler::Style {
                        background: Color::BLACK,
                        background_border_width: 0.0,
                        background_border_color: Color::default(),
                        foreground: Color::WHITE,
                        foreground_border_width: 0.0,
                        foreground_border_color: Color::default(),
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
