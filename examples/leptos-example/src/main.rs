mod app;

use app::App;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    leptos::mount::mount_to_body(App)
}
