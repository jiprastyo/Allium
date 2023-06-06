mod battery;
mod evdev;
mod framebuffer;
mod volume;

use anyhow::Result;
use async_trait::async_trait;

use crate::battery::Battery;
use crate::platform::miyoo::evdev::EvdevKeys;
use crate::platform::miyoo::framebuffer::FramebufferDisplay;
use crate::platform::KeyEvent;
use crate::platform::Platform;

use self::battery::{Miyoo283Battery, Miyoo354Battery};

pub struct MiyooPlatform {
    model: MiyooDeviceModel,
    keys: EvdevKeys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MiyooDeviceModel {
    Miyoo283,
    Miyoo354,
}

#[async_trait(?Send)]
impl Platform for MiyooPlatform {
    type Display = FramebufferDisplay;
    type Battery = Box<dyn Battery>;

    fn new() -> Result<MiyooPlatform> {
        let model = detect_model();

        Ok(MiyooPlatform {
            model,
            keys: EvdevKeys::new()?,
        })
    }

    async fn poll(&mut self) -> Result<Option<KeyEvent>> {
        self.keys.poll().await
    }

    fn display(&mut self) -> Result<FramebufferDisplay> {
        FramebufferDisplay::new()
    }

    fn battery(&self) -> Result<Box<dyn Battery>> {
        Ok(match self.model {
            MiyooDeviceModel::Miyoo283 => Box::new(Miyoo283Battery::new()),
            MiyooDeviceModel::Miyoo354 => Box::new(Miyoo354Battery::new()),
        })
    }

    fn set_volume(&mut self, volume: i32) -> Result<()> {
        match self.model {
            MiyooDeviceModel::Miyoo283 => Ok(()),
            MiyooDeviceModel::Miyoo354 => volume::set_volume(volume),
        }
    }
}

impl Default for MiyooPlatform {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

fn detect_model() -> MiyooDeviceModel {
    if std::path::Path::new("/customer/app/axp_test").exists() {
        MiyooDeviceModel::Miyoo354
    } else {
        MiyooDeviceModel::Miyoo283
    }
}
