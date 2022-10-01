use iced_graphics::renderer::{self, Renderer};
use iced_graphics::{Backend, Primitive};

use iced_native::widget::{self, Widget};
use iced_native::{
    layout, Element, Layout, Length, Point, Rectangle, Size, Vector,
};

#[derive(Default)]
pub struct Rainbow;

impl Rainbow {
    pub fn new() -> Self {
        Self
    }
}

pub fn rainbow() -> Rainbow {
    Rainbow
}

impl<Message, B> Widget<Message, Renderer<B>> for Rainbow
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer<B>,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.width(Length::Fill).resolve(Size::ZERO);

        layout::Node::new(Size::new(20., 200.))
    }

    fn draw(
        &self,
        // _tree: &widget::Tree,
        renderer: &mut Renderer<B>,
        // _theme: &T,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        use iced_graphics::triangle::{Mesh2D, Vertex2D};

        use iced_native::Renderer as _;

        let x = Primitive::Quad {
            bounds: layout.bounds(),
            background: iced::Background::Color(iced::Color::BLACK),
            border_radius: 0.0,
            border_width: 0.1,
            border_color: iced::Color::WHITE,
        };

        renderer.draw_primitive(x);

        // renderer.with_translation(Vector::new(0.0, 0.0), |renderer| {
        //     renderer.draw_primitive(x);
        // });
    }
}

impl<'a, Message, B> From<Rainbow> for Element<'a, Message, Renderer<B>>
where
    B: Backend,
{
    fn from(rainbow: Rainbow) -> Self {
        Self::new(rainbow)
    }
}
