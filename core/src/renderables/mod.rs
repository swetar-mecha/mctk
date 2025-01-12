pub mod circle;
pub mod curve;
pub mod image;
pub mod line;
pub mod radial_gradient;
pub mod rect;
pub mod svg;
pub mod text;
pub mod types;

pub use circle::Circle;
pub use curve::Curve;
pub use image::Image;
pub use line::Line;
pub use radial_gradient::RadialGradient;
pub use rect::Rect;
pub use svg::Svg;
pub use text::Text;

#[derive(Debug, Clone)]
pub enum Renderable {
    Rect(Rect),
    Line(Line),
    Circle(Circle),
    Image(Image),
    Text(Text),
    Svg(Svg),
    RadialGradient(RadialGradient),
    Curve(Curve),
}
