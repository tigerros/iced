use crate::core::{self, Element, Size};
use crate::lazy::component;

#[allow(deprecated)]
pub use crate::lazy::{Component, Lazy, Responsive};
use crate::{Container, container};
use std::hash::Hash;

/// Creates a new [`Lazy`] widget with the given data `Dependency` and a
/// closure that can turn this data into a widget tree.
#[cfg(feature = "lazy")]
pub fn lazy<'a, Message, Theme, Renderer, Dependency, View>(
    dependency: Dependency,
    view: impl Fn(&Dependency) -> View + 'a,
) -> Lazy<'a, Message, Theme, Renderer, Dependency, View>
where
    Dependency: Hash + 'a,
    View: Into<Element<'static, Message, Theme, Renderer>>,
{
    Lazy::new(dependency, view)
}

/// Turns an implementor of [`Component`] into an [`Element`] that can be
/// embedded in any application.
#[cfg(feature = "lazy")]
#[deprecated(
    since = "0.13.0",
    note = "components introduce encapsulated state and hamper the use of a single source of truth. \
    Instead, leverage the Elm Architecture directly, or implement a custom widget"
)]
#[allow(deprecated)]
pub fn component<'a, C, Message, Theme, Renderer>(
    component: C,
) -> Element<'a, Message, Theme, Renderer>
where
    C: Component<Message, Theme, Renderer> + 'a,
    C::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: core::Renderer + 'a,
{
    component::view(component)
}

/// Creates a new [`Responsive`] widget with a closure that produces its
/// contents.
///
/// The `view` closure will be provided with the current [`Size`] of
/// the [`Responsive`] widget and, therefore, can be used to build the
/// contents of the widget in a responsive way.
#[cfg(feature = "lazy")]
pub fn responsive<'a, Message, Theme, Renderer>(
    f: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> Responsive<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
{
    Responsive::new(f)
}

/// Creates a new [`Responsive`] widget that applies the given aspect ratio to a [`Container`]
/// created by the first closure.
///
/// The second closure accepts that [`Container`] as a parameter and outputs the [`Element`]
/// which is actually included in the [`Responsive`].
/// This lets you, for example, apply additional styles.
///
/// # Examples
/// ```no_run
/// //! Sets a 4/1 aspect ratio for a container.
/// use iced_widget::{aspect_ratio, container, Container};
/// use iced_widget::container::bordered_box;
/// aspect_ratio(4.0, || container("Long").style(bordered_box), Container::into)
/// ```
///
/// ```no_run
/// //! Centers a 16/9 "foo" container within the bounds of the [`Responsive`].
/// # use iced_widget::{aspect_ratio, center, container};
/// # use iced_widget::container::bordered_box;
/// aspect_ratio(16.0 / 9.0, || container("Monitor").style(bordered_box), |ratioed| center(ratioed).into())
/// ```
///
/// ```no_run
/// //! Centers a green square with the unused part of the [`Responsive`] being red.
/// # use iced_widget::{aspect_ratio, center, container};
/// use iced_widget::core::{Background, Color};
/// aspect_ratio(
///     1.0,
///     || container("Green square").style(|_| container::Style {
///         text_color: None,
///         background: Some(Background::Color(Color::from_rgb8(0, 100, 0))),
///         border: Default::default(),
///         shadow: Default::default(),
///     }),
///     |ratioed| center(ratioed).style(|_| container::Style {
///         text_color: None,
///         background: Some(Background::Color(Color::from_rgb8(100, 0, 0))),
///         border: Default::default(),
///         shadow: Default::default(),
///     }).into()
/// )
/// ```
#[cfg(feature = "lazy")]
pub fn aspect_ratio<'a, Message, Theme, Renderer>(
    ratio: f32,
    to_ratio: impl Fn() -> Container<'a, Message, Theme, Renderer> + 'a,
    wrapper: impl Fn(
        Container<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer>
    + 'a,
) -> Responsive<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + container::Catalog,
    Renderer: core::Renderer + 'a,
{
    responsive(move |size| {
        wrapper(
            to_ratio()
                .width((size.height * ratio).min(size.width))
                .height((size.width / ratio).min(size.height)),
        )
    })
}
