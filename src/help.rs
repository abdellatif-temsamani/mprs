use prettytable::format;

pub fn help() {
    get_vertion();
    let mut table = table!(
        ["    argument     ", "       description        "],
        ["-----------------", "--------------------------"],
        ["     no args     ", "show stats of current song"],
        ["      play       ", "  play the current song   "],
        ["      pause      ", "  pause the current song  "],
        ["      stop       ", "  stop the current song   "],
        ["      next       ", "    play the next song    "],
        ["      prev       ", " pause the previous song  "],
        ["      help       ", "   shows this help menu   "],
        ["      kill       ", "kill mpd server(Unix only)"],
        ["  --host='HOST'  ", "    host of mpd server    "],
        ["  --port='PORT'  ", "    port of mpd server    "],
        [" --silent or -q  ", "      silent output       "],
        [" --version or -v ", "    print the version     "],
        ["  --help or -h   ", "      print the help      "]
    );

    table.set_format(get_format());
    table.printstd();
}

pub fn get_vertion() {
    println!("MPRS version = {}", env!("CARGO_PKG_VERSION"));
}

fn get_format() -> format::TableFormat {
    format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build()
}
