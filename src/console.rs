use std::default::Default;

use crate::raw::{consoleClear, consoleInit, consoleSelect, consoleSetWindow, PrintConsole};

use crate::gfx::Screen;

pub struct Console {
    context: Box<PrintConsole>,
}

impl Console {
    /// Initialize a console on the chosen screen, overwriting whatever was on the screen
    /// previously (including other consoles). The new console is automatically selected for
    /// printing.
    pub fn init(screen: Screen) -> Self {
        let mut context = Box::new(PrintConsole::default());

        unsafe { consoleInit(screen.into(), context.as_mut()) };

        Console { context }
    }

    /// Select this console as the current target for stdout
    pub fn select(&self) {
        unsafe {
            consoleSelect(self.context.as_ref() as *const _ as *mut _);
        }
    }

    /// Clears all text from the console
    pub fn clear(&self) {
        unsafe { consoleClear() }
    }

    /// Resizes the active console to fit in a smaller portion of the screen.
    ///
    /// The first two arguments are the desired coordinates of the top-left corner
    /// of the console, and the second pair is the new width and height
    ///
    /// This function is unsafe because it does not validate that the input will produce
    /// a console that actually fits on the screen
    pub unsafe fn set_window(&mut self, x: i32, y: i32, width: i32, height: i32) {
        consoleSetWindow(self.context.as_mut(), x, y, width, height);
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        // Get the current console by replacing it with the default.
        let default_console = unsafe { crate::raw::consoleGetDefault() };
        let current_console = unsafe { crate::raw::consoleSelect(default_console) };

        if std::ptr::eq(current_console, &*self.context) {
            // Console dropped while selected. We just replaced it with the
            // default so make sure it's initialized.
            if unsafe { !(*default_console).consoleInitialised } {
                unsafe { crate::raw::consoleInit(Screen::Top.into(), default_console) };
            }
        } else {
            // Console dropped while a different console was selected. Put back
            // the console that was selected.
            unsafe { crate::raw::consoleSelect(current_console) };
        }
    }
}

impl Default for Console {
    fn default() -> Self {
        Console::init(Screen::Bottom)
    }
}
