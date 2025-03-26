use iced::widget::{aspect_ratio, center, column, container};
use iced::{Background, Color, Element};

fn main() {
    iced::run("aspect ratio", AspectRatio::update, AspectRatio::view).unwrap();
}

#[derive(Default)]
struct AspectRatio;

impl AspectRatio {
    fn update(&mut self, _: ()) {}

    fn view(&self) -> Element<()> {
        center(
            column![
                aspect_ratio(
                    1.0,
                    || container("Green 1:1 square").style(|_| container::Style {
                        text_color: None,
                        background: Some(Background::Color(Color::from_rgb8(
                            0, 100, 0
                        ))),
                        border: Default::default(),
                        shadow: Default::default(),
                    }),
                    |ratioed| center(ratioed)
                        .style(|_| container::Style {
                            text_color: None,
                            background: Some(Background::Color(
                                Color::from_rgb8(100, 0, 0)
                            )),
                            border: Default::default(),
                            shadow: Default::default(),
                        })
                        .into()
                ),
                aspect_ratio(
                    16.0 / 9.0,
                    || container("Blue 16:9 container").style(|_| {
                        container::Style {
                            text_color: None,
                            background: Some(Background::Color(
                                Color::from_rgb8(0, 0, 100),
                            )),
                            border: Default::default(),
                            shadow: Default::default(),
                        }
                    }),
                    |ratioed| center(ratioed).into()
                ),
            ]
            .spacing(20),
        )
        .padding(20)
        .into()
    }
}
