use std::fs;
use std::path::Path;

pub fn load_templates() -> String {
    let css = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/styles/main.css"),
    )
    .expect("Failed to load CSS");

    let home_html = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/templates/home.html"),
    )
    .expect("Failed to load home template");

    format!(
        r#"
        <style>{}</style>
        {}
        "#,
        css, home_html
    )
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_PAGE: String = load_templates();
}
