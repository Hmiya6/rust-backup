mod app;
pub use app::App;

pub mod config;
// pub use config::Config;


fn main() {
    let app = App::new(std::path::Path::new("test_config.toml")).unwrap();
    app.run();
    println!("hello world")
}
