use kovi::build_bot;
fn main() {
    kovi::set_logger();
    build_bot!().run();
}
