
fn main() {
    //let args: Vec<String> = std::env::args().collect();

    //Print version info
    println!("{}\n\n", vsemur::about::version::string());
    //println!("{}\n\n", vsemur::about::LICENSE);

    println!("Hello, world! (cli)");
}
