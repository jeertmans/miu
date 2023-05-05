use cargo::ops::install_list;
use cargo::util::config::Config;

fn main() {
    let config = Config::default().unwrap();
    install_list(None, &config).unwrap();
    println!("Hello, world!");
}
