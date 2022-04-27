use anyhow::{anyhow, Result};
use sifis::Thing;

use crate::{ConnectedObject, Percentage};

pub struct Oven(Thing);

impl ConnectedObject for Oven {
    const AT_TYPE: &'static str = "Oven";
}

impl TryFrom<Thing> for Oven {
    type Error = &'static str;

    fn try_from(t: Thing) -> Result<Self, Self::Error> {
        if t.has_attype(Oven::AT_TYPE) {
            Ok(Oven(t))
        } else {
            Err("The Thing is not a Oven!")
        }
    }
}

impl Oven {
    /// Turns an oven on.
    ///
    /// # Hazards
    ///
    /// * Fire hazard\
    ///   The execution may cause fire
    /// * Electric energy consumption\
    ///   The execution enables a device that consumes electricity
    pub fn turn_oven_on(&mut self, temperature: Percentage) -> Result<()> {
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("OnOff"))
            .map(|p| p.set(true).ok())
            .flatten()
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("Temperature"))
            .map(|p| p.set(temperature.0).ok())
            .flatten()
            .ok_or(anyhow!("Error"))
    }

    /// Turns an oven off.
    pub fn turn_oven_off(&mut self) -> Result<()> {
        self.0
            .properties
            .values()
            .find(|p| p.has_attype("OnOff"))
            .map(|p| p.set(false).ok())
            .flatten()
            .ok_or(anyhow!("Error"))
    }
}
