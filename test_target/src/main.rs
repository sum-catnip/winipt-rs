fn main() {
    println!("Hello, world!");
    // well get killed anyways
    std::thread::sleep(std::time::Duration::from_secs(10000))
}
