use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::widget::Widget;
use iced_native::{Color, Element, Length, Point, Rectangle, Size};

// struct LefChannel {
//     width: f32,
//     height: f32,
// }

// struct RightChannel {
//     width: f32,
//     height: f32,
// }

pub struct LevelMeter {
    width: f32,
    height: f32,
}

impl LevelMeter {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

pub fn level_meter(width: f32, height: f32) -> LevelMeter {
    LevelMeter::new(width, height)
}

impl<Message, Renderer> Widget<Message, Renderer> for LevelMeter
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        // _state: &State,
        renderer: &mut Renderer,
        // _theme: &Renderer,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        // println!(
        //     "VIEWPORT{:#?} |||| Cursor{:#?}",
        //     _viewport, _cursor_position
        // );

        let quad = renderer::Quad {
            bounds: layout.bounds(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        renderer.fill_quad(quad, Color::from_rgba(0., 1., 0., 0.7));
    }
}

impl<'a, Message, Renderer> From<LevelMeter> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(level_meter: LevelMeter) -> Self {
        Self::new(level_meter)
    }
}

// Black background container

// pub mod level_meter_container_mod {

//     use iced_native::layout::{self, Layout};
//     use iced_native::renderer;
//     use iced_native::widget::Widget;
//     use iced_native::{Color, Element, Length, Point, Rectangle, Size};

//     pub struct LevelMeterContainer {
//         width: f32,
//         height: f32,
//     }

//     impl LevelMeterContainer {
//         pub fn new(width: f32, height: f32) -> Self {
//             Self { width, height }
//         }
//     }

//     pub fn level_meter_container(
//         width: f32,
//         height: f32,
//     ) -> LevelMeterContainer {
//         LevelMeterContainer::new(width, height)
//     }

//     impl<Message, Renderer> Widget<Message, Renderer> for LevelMeterContainer
//     where
//         Renderer: renderer::Renderer,
//     {
//         fn width(&self) -> Length {
//             Length::Shrink
//         }

//         fn height(&self) -> Length {
//             Length::Shrink
//         }

//         fn layout(
//             &self,
//             _renderer: &Renderer,
//             _limits: &layout::Limits,
//         ) -> layout::Node {
//             layout::Node::new(Size::new(self.width, self.height))
//         }

//         fn draw(
//             &self,
//             // _state: &State,
//             renderer: &mut Renderer,
//             // _theme: &Renderer,
//             _style: &renderer::Style,
//             layout: Layout<'_>,
//             _cursor_position: Point,
//             _viewport: &Rectangle,
//         ) {
//             // println!(
//             //     "VIEWPORT{:#?} |||| Cursor{:#?}",
//             //     _viewport, _cursor_position
//             // );

//             let quad = renderer::Quad {
//                 bounds: layout.bounds(),
//                 border_radius: 0.0,
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             };

//             renderer.fill_quad(quad, Color::BLACK);
//         }
//     }

//     impl<'a, Message, Renderer> From<LevelMeterContainer>
//         for Element<'a, Message, Renderer>
//     where
//         Renderer: renderer::Renderer,
//     {
//         fn from(level_meter_container: LevelMeterContainer) -> Self {
//             Self::new(level_meter_container)
//         }
//     }
// }
