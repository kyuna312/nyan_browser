use std::fs;
use std::path::Path;

pub fn load_templates() -> (String, String) {
    let css = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/styles/main.css"),
    )
    .expect("Failed to load CSS");

    let home_html = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/templates/home.html"),
    )
    .expect("Failed to load home template");

    let header_html = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/templates/header.html"),
    )
    .expect("Failed to load header template");

    let default_page = format!("<style>{}</style>\n{}", css, home_html);

    let custom_header = format!("<style>{}</style>\n{}", css, header_html);

    (default_page, custom_header)
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: (String, String) = load_templates();
    pub static ref DEFAULT_PAGE: &'static str = &TEMPLATES.0;
    pub static ref CUSTOM_HEADER: &'static str = &TEMPLATES.1;
}
