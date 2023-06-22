#[cfg(target_arch = "arm")]
mod miyoo;
#[cfg(not(target_arch = "arm"))]
mod simulator;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    battery::Battery,
    display::{settings::DisplaySettings, Display},
};

#[cfg(target_arch = "arm")]
pub type DefaultPlatform = miyoo::MiyooPlatform;

#[cfg(not(target_arch = "arm"))]
pub type DefaultPlatform = simulator::SimulatorPlatform;

// Platform is not threadsafe because it is ?Send
#[async_trait(?Send)]
pub trait Platform {
    type Display: Display;
    type Battery: Battery;

    fn new() -> Result<Self>
    where
        Self: Sized;

    fn display(&mut self) -> Result<Self::Display>;

    fn battery(&self) -> Result<Self::Battery>;

    async fn poll(&mut self) -> KeyEvent;

    fn set_volume(&mut self, volume: i32) -> Result<()>;

    fn get_brightness(&self) -> Result<u8>;

    fn set_brightness(&mut self, brightness: u8) -> Result<()>;

    fn set_display_settings(&mut self, settings: &DisplaySettings) -> Result<()>;

    fn device_model() -> String;

    fn firmware() -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
    Autorepeat(Key),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    Start,
    Select,
    L,
    R,
    Menu,
    L2,
    R2,
    Power,
    VolDown,
    VolUp,
    Unknown,
}
