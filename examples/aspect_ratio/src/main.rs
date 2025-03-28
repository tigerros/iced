use iced::widget::{button, center, column, container, responsive, row};
use iced::{Background, Color, Element, Length, Size};

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
                responsive(move |size| {
                    let row_height = 40.0;
                    let square_available =
                        Size::new(size.width, size.height - row_height);
                    let square_size = square_available.aspect_ratio(1.0);

                    column![
                        container("Green 1:1 square with buttons")
                            .style(|_| container::Style {
                                text_color: None,
                                background: Some(Background::Color(
                                    Color::from_rgb8(0, 100, 0)
                                )),
                                border: Default::default(),
                                shadow: Default::default(),
                            })
                            .width(square_size.width)
                            .height(square_size.height),
                        row![
                            button("<").width(Length::Fill),
                            button(">").width(Length::Fill),
                        ]
                        .width(square_size.width)
                        .height(row_height)
                    ]
                    .into()
                }),
                responsive(move |size| {
                    let ratioed = size.aspect_ratio(16.0 / 9.0);

                    center(
                        container("Blue 16:9 container")
                            .style(|_| container::Style {
                                text_color: None,
                                background: Some(Background::Color(
                                    Color::from_rgb8(0, 0, 100),
                                )),
                                border: Default::default(),
                                shadow: Default::default(),
                            })
                            .width(ratioed.width)
                            .height(ratioed.height),
                    )
                    .style(|_| container::Style {
                        text_color: None,
                        background: Some(Background::Color(Color::from_rgb8(
                            0, 0, 80,
                        ))),
                        border: Default::default(),
                        shadow: Default::default(),
                    })
                    .into()
                }),
            ]
            .spacing(20),
        )
        .padding(20)
        .into()
    }
}
