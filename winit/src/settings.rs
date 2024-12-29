//! Configure your application.
use crate::core;

use std::borrow::Cow;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

/// The settings of an application.
#[derive(Debug, Clone, Default)]
pub struct Settings {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,

    #[cfg(target_os = "android")]
    /// The Android App
    pub android_app: Option<AndroidApp>
}

impl From<core::Settings> for Settings {
    fn from(settings: core::Settings) -> Self {
        Self {
            id: settings.id,
            fonts: settings.fonts,
            #[cfg(target_os = "android")]
            android_app: None
        }
    }
}
