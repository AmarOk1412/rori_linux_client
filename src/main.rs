extern crate qmlrs;

fn main() {
    let mut engine = qmlrs::Engine::new();
    engine.load_local_file("ui/rori.qml");
    engine.exec();
}
