
fn main() {
    //Print version info
    println!("{}\n\n", vsemur::about::version::string());

    //Handle command line arguments
    if std::env::args().len() != 2 {
        println!("Error: Expected 1 argument (path to rom or --version)\n");
        return;
    }
    let arg: String = std::env::args().nth(1).unwrap();
    if arg == "--version" {
        println!("{}", vsemur::about::LICENSE);
        return
    }

    let state: vsemur::State = vsemur::State::new();

    println!("Hello, world! (cli)");//TODO other things here
}
