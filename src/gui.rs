/* vsemur-gui
 * By: John Jekel
 *
 * libvsemur GUI frontend
 *
*/

//!libvsemur GUI frontend

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

const gtk4_rs_license: &str = "
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
        2 => {
            if std::env::args().nth(1).unwrap() == "--version" {
                eprintln!("{}", vsemur::about::LICENSE);

                eprintln!("\n\nIn addition, vsemur-gui uses the gtk4-rs project for displaying GUI elements:\n {}", gtk4_rs_license);
                return;
            } else {
                eprintln!("\x1b[31mError: Invalid argument\x1b[0m\n");
                return;
            }
        },
        3 => {},//Continue to the next part of main()
        _ => {
            eprintln!("\x1b[31mError: Expected 1 or 2 arguments (path to bios, path to rom; or --version)\x1b[0m\n");
            return;
        },
    }

    eprintln!("Hello, world! (gui)");//TODO other things here
}
