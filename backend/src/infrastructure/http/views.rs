use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use maud::{DOCTYPE, Markup, PreEscaped, html};

const LOGO_BYTES: &[u8] = include_bytes!("logo.png");
const FAVICON_BYTES: &[u8] = include_bytes!("favicon.ico");

pub fn layout(title: &str, content: Markup) -> Markup {
    let favicon = STANDARD.encode(FAVICON_BYTES);
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }

                // SEO Meta Tags
                meta name="description" content="Generate high-quality QR codes instantly with K-QR. A fast, secure, and easy-to-use QR code generator built with Rust.";
                meta name="keywords" content="qr code generator, rust, k-qr, fast qr, secure qr code, htmx qr generator";
                meta name="author" content="Gabriel Kaszewski";
                meta name="robots" content="index, follow";

                // Open Graph / Facebook
                meta property="og:type" content="website";
                meta property="og:title" content=(title);
                meta property="og:description" content="A high-performance, secure QR code generator built with Rust and HTMX.";
                meta property="og:site_name" content="K-QR";

                // Twitter Card
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(title);
                meta name="twitter:description" content="Generate high-quality QR codes instantly with K-QR. Built with Rust for performance.";

                link rel="icon" type="image/x-icon" href=(format!("data:image/x-icon;base64,{}", favicon));
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet";
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                style {
                    (PreEscaped(include_str!("style.css")))
                }
            }
            body {
                (content)
            }
        }
    }
}

fn logo_base64() -> String {
    STANDARD.encode(LOGO_BYTES)
}

pub fn index_page() -> Markup {
    let logo = logo_base64();
    layout(
        "K-QR - QR Code Generator",
        html! {
            div.container {
                img.logo src=(format!("data:image/png;base64,{}", logo)) alt="K-QR Logo";
                h1 { "K-QR" }
                p.subtitle { "Generate QR codes instantly" }
                form hx-get="/api/qr" hx-target="#result" hx-indicator=".container" {
                    div.input-group {
                        label for="data" { "Enter URL or text" }
                        input type="text" name="data" id="data" placeholder="https://example.com" required autocomplete="off";
                    }
                    button type="submit" { "Generate QR Code" }
                }
                div id="loading" class="htmx-indicator" { "Generating..." }
                div id="result" {}
            }
        },
    )
}

pub fn qr_component(image_data: &[u8]) -> Markup {
    let base64_str = STANDARD.encode(image_data);
    html! {
        div.qr-result {
            img src=(format!("data:image/png;base64,{}", base64_str)) alt="Generated QR Code";
        }
    }
}
