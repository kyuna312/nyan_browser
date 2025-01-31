use once_cell::sync::Lazy;
use std::path::Path;
use tokio::fs;

pub static DEFAULT_PAGE: Lazy<String> = Lazy::new(|| {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let (css, html) = tokio::join!(
            fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/styles/main.css")
            ),
            fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/templates/home.html")
            )
        );

        format!(
            r#"<style>{}</style>{}"#,
            css.expect("Failed to load CSS"),
            html.expect("Failed to load HTML")
        )
    })
});
