extern crate notify_rust;
#[macro_use]
extern crate clap;

use std::io::Write;

use notify_rust::Notification;
use clap::{App, SubCommand, Arg};

arg_enum!{
pub enum NotificationUrgency{Low, Normal, Critical}
}

fn main() {
    let urgencies = ["low","normal","high"];

    let matches = App::new("notify-rust")
                        .version(&crate_version!()[..])
                        .author("Hendrik Sollich <hendrik@hoodie.de>")
                        .about("notify-send in rust")
                        .subcommand(SubCommand::with_name("send")
                                    .about("Shows a notification")
                                    // {{{

                                    .arg( Arg::with_name("summary")
                                          .help("Title of the Notification.")
                                          .required(true))

                                    .arg( Arg::with_name("body")
                                          .help("Message body"))

                                    .arg( Arg::with_name("app-name")
                                          .help("Set a specific app-name manually.")
                                          .short("a")
                                          .long("app-name")
                                          .takes_value(true))

                                    .arg( Arg::with_name("expire-time")
                                          .help("Time until expiration in milliseconds. 0 means forever. ")
                                          .short("t")
                                          .long("expire-time")
                                          .takes_value(true))

                                    .arg( Arg::with_name("icon")
                                          .short("i")
                                          .help("Icon of notification.")
                                          .long("icon")
                                          .takes_value(true))

                                    .arg( Arg::with_name("category")
                                          .help("Set a category.")
                                          .short("c")
                                          .long("category")
                                          .takes_value(true))

                                    .arg( Arg::with_name("urgency")
                                          .help("How urgent is it.")
                                          .short("u")
                                          .long("urgency")
                                          .takes_value(true)
                                          .possible_values(&urgencies))

                                    .arg( Arg::with_name("debug")
                                          .help("Also prints notification to stdout")
                                          .short("d")
                                          .long("debug"))
                                    //}}}
                                    )
                        .subcommand(SubCommand::with_name("info")
                                    .about("Shows information about the running notification server")
                                    )
                        .subcommand(SubCommand::with_name("server")
                                    .about("Starts a little notification server for testing")
                                    )
                        .arg_required_else_help(true)

                        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("server")
    {
        use std::thread;

        use notify_rust::Notification;
        use notify_rust::server::NotificationServer;
        let mut server = NotificationServer::new();
        thread::spawn(move||{
            server.start( |notification| println!("{:#?}", notification))
        });

        println!("Press enter to exit.\n");

        std::thread::sleep_ms(1_000);

        Notification::new()
            .summary("Notification Logger")
            .body("If you can read this in the console, the server works fine.")
            .show().expect("Was not able to send initial test message");

        let mut _devnull = String::new();
        let _ = std::io::stdin().read_line(&mut _devnull);
        println!("Thank you for choosing notify-rust.");
    }



    else if let Some(_matches) = matches.subcommand_matches("info")
    {
        println!("server information:\n {:?}\n", notify_rust::get_server_information().unwrap());
        println!("capabilities:\n {:?}\n", notify_rust::get_capabilities().unwrap());
    }



    else if let Some(matches) = matches.subcommand_matches("send")
    {
        let mut notification = Notification::new();

        let summary = matches.value_of("summary").unwrap();
        notification.summary(summary);

        if let Some(appname) = matches.value_of("app-name"){
            notification.appname(appname);
        }

        if let Some(icon) = matches.value_of("icon"){
            notification.icon(icon);
        }

        if let Some(body) = matches.value_of("body"){
            notification.body(body);
        }

        if let Some(categories) = matches.value_of("category"){
            //notification.body(body);
            for category in categories.split(':'){
                notification.hint(notify_rust::NotificationHint::Category(category.to_owned()));
            }
        }

        if let Some(timeout_string) = matches.value_of("expire-time"){
            if let Ok(timeout) = timeout_string.parse::<i32>(){
                notification.timeout(timeout); }
            else {println!("can't parse timeout {:?}, please use a number", timeout_string);}
        }

        if matches.is_present("urgency") {
            let urgency = value_t_or_exit!(matches.value_of("urgency"), NotificationUrgency);
            // TODO: somebody make this a cast, please!
            match urgency {
                NotificationUrgency::Low      => notification.urgency(notify_rust::NotificationUrgency::Low),
                NotificationUrgency::Normal   => notification.urgency(notify_rust::NotificationUrgency::Normal),
                NotificationUrgency::Critical => notification.urgency(notify_rust::NotificationUrgency::Critical),
            };
        }

        if matches.is_present("debug"){
            if notification.show_debug().is_err()
            {
                writeln!(&mut std::io::stderr(),
                "Sending this notification was not possible.").unwrap();
            }

        } else {
            if notification.show().is_err()
            {
                writeln!(&mut std::io::stderr(),
                "Sending this notification was not possible.").unwrap();
            }
        }
    }
}
