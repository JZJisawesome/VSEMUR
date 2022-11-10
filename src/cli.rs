
fn main() {
    //Print version info
    eprintln!("VSEMUR Command-Line Interface");
    eprintln!("Powered by: {}\n", vsemur::about::version::pretty_string());

    //Handle command line arguments
    if std::env::args().len() != 2 {
        eprintln!("\x1b[31mError: Expected 1 argument (path to rom or --version)\x1b[0m\n");
        return;
    }
    let arg: String = std::env::args().nth(1).unwrap();
    if arg == "--version" {
        eprintln!("{}", vsemur::about::LICENSE);
        return
    }

    let mut state: vsemur::State = vsemur::State::new();
    //TODO load rom

    loop {
        match vsemur::tick(&mut state) {
            vsemur::ReturnCode::OK => { continue; }
            vsemur::ReturnCode::FAIL => {//This should never occur
                panic!("\x1b[31mError: Tick failed\x1b[0m");
            }
            vsemur::ReturnCode::EXIT_NORMAL => {
                eprintln!("Normal exit. Bye!");
                return;
            }
        }
    }
}
