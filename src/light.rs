use anyhow::{anyhow, Result};
use sifis::Thing;

use crate::{ConnectedObject, Percentage, Rgb};

pub struct Light(Thing);

impl ConnectedObject for Light {
    const AT_TYPE: &'static str = "Light";
}

impl TryFrom<Thing> for Light {
    type Error = &'static str;

    fn try_from(t: Thing) -> Result<Self, Self::Error> {
        if t.has_attype(Light::AT_TYPE) {
            Ok(Light(t))
        } else {
            Err("The Thing is not a Light!")
        }
    }
}

impl Light {
    /// Turns a light on.
    ///
    /// # Hazards
    ///
    /// * Fire hazard\
    ///   The execution may cause fire
    pub fn turn_light_on(&mut self, brightness: Percentage, color: Rgb) -> Result<()> {
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("OnOff"))
            .and_then(|p| p.set(true).ok())
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("BrightnessProperty"))
            .and_then(|p| p.set(brightness.0).ok())
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("ColorProperty"))
            .and_then(|p| p.set(&color.to_string()).ok())
            .ok_or(anyhow!("Error"))
    }

    /// Turns a light off.
    pub fn turn_light_off(&mut self) -> Result<()> {
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("OnOff"))
            .and_then(|p| p.set(false).ok())
            .ok_or(anyhow!("Error"))
    }
}
