
fn main() {
    //Print version info
    eprintln!("VSEMUR GUI");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    let arg: String;
    match std::env::args().len() {
        2 => {
            if std::env::args().nth(1).unwrap() == "--version" {
                eprintln!("{}", vsemur::about::LICENSE);
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
