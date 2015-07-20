# notify-rust

A commandline tool that shows desktop notifications using [notify-rust](http://hoodie.github.io/notify-rust/.)

```
notify-rust-cli 0.1.0
Hendrik Sollich <hendrik@hoodie.de>
notify-send in rust

USAGE:
	notify-rust-cli [FLAGS] [OPTIONS] [ARGS] <summary>

FLAGS:
    -d, --debug      Also prints notification to stdout
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --app-name <app-name>              Set a specific app-name manually.
    -c, --category <category>              Set a category.
    -t, --expire-time <expire-time>        Time until expiration in milliseconds. 0 means forever. 
    -i, --icon <icon>                      Icon of notification.
    -u, --urgency <urgency>                How urgent is it. [values: high, low, normal]

ARGS:
    summary      Title of the Notification.
    body         Message body

```
