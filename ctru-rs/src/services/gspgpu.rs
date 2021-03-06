//! GSPGPU service

use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Psc0,
    Psc1,
    VBlank0,
    VBlank1,
    PPF,
    P3D,
    DMA,
}

/// The different framebuffer formats supported by the 3DS
#[derive(Copy, Clone, Debug)]
pub enum FramebufferFormat {
    /// RGBA8. 4 bytes per pixel
    Rgba8,
    /// BGR8. 3 bytes per pixel
    Bgr8,
    /// RGB565. 2 bytes per pixel
    Rgb565,
    /// RGB5A1. 2 bytes per pixel
    Rgb5A1,
    /// RGBA4. 2 bytes per pixel
    Rgba4,
}

impl FramebufferFormat {
    /// Returns the number of bytes per pixel used by this FramebufferFormat
    pub fn pixel_depth_bytes(&self) -> usize {
        use self::FramebufferFormat::*;
        match *self {
            Rgba8 => 4,
            Bgr8 => 3,
            Rgb565 => 2,
            Rgb5A1 => 2,
            Rgba4 => 2,
        }
    }
}

/// Waits for a GSPGPU event to occur.
///
/// `discard_current` determines whether to discard the current event and wait for the next event
pub fn wait_for_event(ev: Event, discard_current: bool) {
    unsafe {
        ctru_sys::gspWaitForEvent(ev.into(), discard_current);
    }
}

impl From<ctru_sys::GSPGPU_FramebufferFormat> for FramebufferFormat {
    fn from(g: ctru_sys::GSPGPU_FramebufferFormat) -> Self {
        use self::FramebufferFormat::*;
        match g {
            ctru_sys::GSP_RGBA8_OES => Rgba8,
            ctru_sys::GSP_BGR8_OES => Bgr8,
            ctru_sys::GSP_RGB565_OES => Rgb565,
            ctru_sys::GSP_RGB5_A1_OES => Rgb5A1,
            ctru_sys::GSP_RGBA4_OES => Rgba4,
            _ => unreachable!(),
        }
    }
}

impl From<FramebufferFormat> for ctru_sys::GSPGPU_FramebufferFormat {
    fn from(g: FramebufferFormat) -> Self {
        use self::FramebufferFormat::*;
        match g {
            Rgba8 => ctru_sys::GSP_RGBA8_OES,
            Bgr8 => ctru_sys::GSP_BGR8_OES,
            Rgb565 => ctru_sys::GSP_RGB565_OES,
            Rgb5A1 => ctru_sys::GSP_RGB5_A1_OES,
            Rgba4 => ctru_sys::GSP_RGBA4_OES,
        }
    }
}

impl From<Event> for ctru_sys::GSPGPU_Event {
    fn from(ev: Event) -> Self {
        use self::Event::*;
        match ev {
            Psc0 => ctru_sys::GSPGPU_EVENT_PSC0,
            Psc1 => ctru_sys::GSPGPU_EVENT_PSC1,
            VBlank0 => ctru_sys::GSPGPU_EVENT_VBlank0,
            VBlank1 => ctru_sys::GSPGPU_EVENT_VBlank1,
            PPF => ctru_sys::GSPGPU_EVENT_PPF,
            P3D => ctru_sys::GSPGPU_EVENT_P3D,
            DMA => ctru_sys::GSPGPU_EVENT_DMA,
        }
    }
}
