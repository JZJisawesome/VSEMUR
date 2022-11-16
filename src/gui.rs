/* vsemur-gui
 * By: John Jekel
 *
 * libvsemur GUI frontend
 *
*/

//!libvsemur GUI frontend

/* Imports */

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Text};

/* Constants */

const APP_ID: &str = "ca.jekel.vsemur";

const GTK4_RS_LICENSE: &str = "
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.";

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

fn main() {
    //Print version info
    eprintln!("VSEMUR GUI");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    match std::env::args().len() {
        1 => {},//Continue to the next part of main()
        2 => {
            if std::env::args().nth(1).unwrap() == "--version" {
                eprintln!("{}", vsemur::about::LICENSE);

                eprintln!("\n\nIn addition, vsemur-gui uses the gtk4-rs project for displaying GUI elements:\n {}", GTK4_RS_LICENSE);
                return;
            } else {
                eprintln!("\x1b[31mError: Invalid argument\x1b[0m\n");
                return;
            }
        },
        _ => {
            eprintln!("\x1b[31mError: Expected <= 1 argument (--version or nothing)\x1b[0m\n");
            return;
        },
    }

    //Create a new GTK application and create the window
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();

    eprint!("abcd");//Testing

    //TODO emulation loop here
}

fn build_ui(app: &Application) {

    //let text = Text::builder()
    //    .build();

        // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

        button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });


    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("VSEMUR GUI")
        //.child(&text)
        .child(&button)
        .build();

    // Present window
    window.present();
}
