use std::io::Write;
use std::net::TcpListener;

#[macro_export]
macro_rules! artm {
($file_path:expr) => {
    const ALZN_CONTENT: &str = include_str!($file_path);

    pub fn render() -> String {
        let html = $crate::extract_block(ALZN_CONTENT, "<htmlcode>", "</htmlcode>");
        let style = $crate::extract_block(ALZN_CONTENT, "<style>", "</style>");
        let script = $crate::extract_block(ALZN_CONTENT, "<script rust>", "</script>");

        let mut page = String::new();
        page.push_str("<!DOCTYPE html><html><head><style>");
        page.push_str(&style);
        page.push_str("</style><script type='application/wasm-script'>");
        page.push_str(&script);
        page.push_str("</script></head><body>");
        page.push_str(&html);
        page.push_str("</body></html>");

        page
    }
};

    (
        <script rust> $($script:tt)* </script>
        <htmlcode> $($html:tt)* </htmlcode>
        <style> $($style:tt)* </style>
    ) => {
        pub fn render() -> String {
            let script_content = stringify!($($script)*);
            let html_body = stringify!($($html)*);
            let css_styles = stringify!($($style)*);

            format!(
                "<!DOCTYPE html><html><head><style>{}</style>\
                <script type='application/wasm-script'>{}</script></head>\
                <body>{}</body></html>",
                css_styles, script_content, html_body
            )
        }
    };
}


pub fn extract_block(content: &str, open_tag: &str, close_tag: &str) -> String {
    if let Some(start_idx) = content.find(open_tag) {
        if let Some(end_idx) = content.find(close_tag) {
            let start = start_idx + open_tag.len();
            return content[start..end_idx].trim().to_string();
        }
    }
    String::new()
}

pub fn start_server(html_content: String) {
    let listener = TcpListener::bind("127.0.0.1:5090").unwrap();
    println!("Aluzin Server running on http://127.0.0.1:5090");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                html_content.len(),
                html_content
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}

fn main() {
    println!("Aluzin has been started");
}
