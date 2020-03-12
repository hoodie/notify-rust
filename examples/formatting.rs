#![allow(unused_must_use)]
use notify_rust::Notification;

fn main() {
    Notification::new()
        .summary("Formatting")
        .appname("chromium")
        .body(&format!("This is not chrome, but <b>{}</b>!<br/>", "bold"))
        .icon("chromium")
        .show();

    Notification::new()
        .summary("Table Test - will probably not work")
        .body(&format!(
            "<table><tr><td>{}</td><td>cell 2</td></tr> <tr><td>cell 3</td><td>cell 4</td></tr></table>",
            "cell 1"
        ))
        .icon("table")
        .show();

    Notification::new()
        .summary("linebreaks")
        .body(
            "This should be a paragraph, this is why I wrote more than one line worth of text, only to make it linebreak.
             This should be another paragraph."
        )
        .icon("thunderbird")
        .show();
    Notification::new()
        .summary("Paragraphs")
        .body(
            "<p>This should be a paragraph, this is why I wrote more than one line worth of text, only to make it linebreak.</p>
             <p>This should be another paragraph.</p> <p>This should be another paragraph.</p>"
        )
        .icon("thunderbird")
        .show();
}
