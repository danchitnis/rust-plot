//#![windows_subsystem = "windows"]

extern crate web_view;

use web_view::*;

fn main() {
    web_view::builder()
        .title("Page load example")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

const HTML: &str = r#"
<!doctype html>
<html>
	<body class="body">
      <h1>Hello World!</h1>
      <button>OK!🥗</button>
    </body>
    <style>
        .body {
            background-color: rgb(10,10,10);
            color: rgb(245,245,245);
        }
    </style>
</html>
"#;