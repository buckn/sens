extern crate download_rs;

use download_rs::async_download::Download;

fn main() {
    let mut download = Download::new(
        "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css",
        Some("src/css/fa.min.css"),
        None,
    );
    match download.download() {
        Ok(_) => println!("Downloaded Font Awesome"),
        Err(e) => println!("Font Awesome Error ： {}", e.to_string()),
    }
    download = Download::new(
        "https://cdn.jsdelivr.net/npm/bulma@0.8.0/css/bulma.min.css",
        Some("src/css/bulma.min.css"),
        None,
    );
    match download.download() {
        Ok(_) => println!("Downloaded Bulma"),
        Err(e) => println!("Bulma Error ： {}", e.to_string()),
    }
}
