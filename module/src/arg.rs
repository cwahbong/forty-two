use clap::Arg;

pub fn address<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("address")
        .short("a")
        .long("addr")
        .takes_value(true)
}

pub fn port<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("port")
        .short("p")
        .long("port")
        .takes_value(true)
}

pub fn request_name<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("request name")
        .short("n")
        .long("name")
        .takes_value(true)
}

pub fn request_args<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("request arguments")
        .short("r")
        .long("args")
        .takes_value(true)
}
