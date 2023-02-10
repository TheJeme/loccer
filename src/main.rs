use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = loccer::Config::init(&args);
    loccer::run(config);
}
