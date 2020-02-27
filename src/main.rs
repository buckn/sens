//#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use web_view::*;

fn main() {
    let html = format!(
        r#"
{doctype}
<html>
    <head>
        {head}
        {styles}
        {scripts}
    </head>
    <body>
        {body}
    </body>
</html>"#,
        doctype = include_str!("html/doctype.html"),
        head = include_str!("html/head.html"),
        body = include_str!("html/body.html"),
        styles = inline_style(include_str!("css/index.css")),
        scripts = inline_script(include_str!("js/index.js")),
        );

    println!("{}", html);

    let mut webview = web_view::builder()
    .title("sens")
    .content(Content::Html(html))
    .size(500, 500)
    .resizable(true)
    .debug(true)
    .user_data(())
    .invoke_handler(|_webview, _arg| Ok(()))
    .build()
    .unwrap();

    webview.set_color((156, 39, 176));

    let res = webview.run().unwrap();

    println!("final state: {:?}", res);
}

fn render(webview: &mut WebView<Vec<Task>>) -> WVResult {
    let render_tasks = {
        let tasks = webview.user_data();
        println!("{:#?}", tasks);
        format!("rpc.render({})", serde_json::to_string(tasks).unwrap())
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}