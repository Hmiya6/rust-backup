
// modules -----------------------------
mod app;
pub use app::App;

pub mod config;

mod backup;
pub use backup::Backup;
// -------------------------------------

use std::path::Path;


// main --------------------------------
fn main() {
    let mut app = App::new(Path::new("rust-backup-config.toml")).unwrap();
    app.run();
    println!("See you next time");
}
// -------------------------------------
