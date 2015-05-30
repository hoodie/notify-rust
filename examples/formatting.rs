extern crate notify_send;
use notify_send::Notification;
fn main()
{

    // use it this way
    Notification::new()
        .summary("Chromium Crashed")
        .appname("chromium")
        .body(&format!("This is <b>{}</b>!<br/>", "a lie"))
        .icon("chromium")
        .timeout(6000) //miliseconds
        .send();


    // use it this way
    Notification::new()
        .summary("Table Test - will probably not work")
        .body(&format!("<table>
                       <tr><td>{}</td><td>cell 2</td></tr> <tr><td>cell 3</td><td>cell 4</td></tr>
                       </table>", "cell 1"))
        .icon("table")
        .timeout(6000) //miliseconds
        .send();


}

