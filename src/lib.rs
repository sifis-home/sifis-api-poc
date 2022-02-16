mod context;
mod light;
mod oven;

pub use context::*;
pub use light::*;
pub use oven::*;

/// Define the type of a connected object.
pub trait ConnectedObject {
    /// Connected object type
    const AT_TYPE: &'static str;
}

/// Percentage in the range [0, 100].
pub struct Percentage(u8);

impl Percentage {
    /// Creates a new `Percentage`.
    pub fn new(val: u8) -> Self {
        Self(val.min(100))
    }
}

/// Colour of a light.
pub struct Rgb(u8, u8, u8);

impl ToString for Rgb {
    fn to_string(&self) -> String {
        "#".to_owned()
            + &format!("{:02X}", self.0)
            + &format!("{:02X}", self.1)
            + &format!("{:02X}", self.2)
    }
}

impl Rgb {
    /// Creates a new `Rgb`.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}
