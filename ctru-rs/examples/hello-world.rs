use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    // Initialize ctrulib service handles.
    // Service handles are internally reference-counted. When all instances of a
    // service handle go out of scope, the service will be closed.
    ctru::init();

    // The APT service handles application management functions, such as enabling sleep
    // mode and jumping to the home menu or to other applications
    let apt = Apt::init().unwrap();

    // The HID service handles button and touch screen inputs.
    let hid = Hid::init().unwrap();

    // The GFX service manages the framebuffers for the top and bottom screens.
    let gfx = Gfx::default();

    // Initialize a ctrulib console and direct standard output to it.
    // Consoles can be initialized on both the top and bottom screens.
    let _console = Console::init(&gfx, ctru::gfx::Screen::Top);

    // Now we can print to stdout!
    println!("Hello, world!");

    // We can use escape sequences to move the cursor around the terminal.
    // The following text will be moved down 29 rows and right 16 characters
    // before printing begins.
    println!("\x1b[29;16HPress Start to exit");

    // Main application loop.
    while apt.main_loop() {
        // Flushes and swaps the framebuffers when double-buffering
        // is enabled
        gfx.flush_buffers();
        gfx.swap_buffers();

        // Wait for the next frame to begin
        gfx.wait_for_vblank();

        // Scan for user input.
        hid.scan_input();

        // Check if the user has pressed the given button on this frame.
        // If so, break out of the loop.
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }

    // All of our service handles will drop out of scope at this point,
    // triggering the end of our application.
}
