use colored::Colorize;
use prettytable::format;

pub fn help() {
    get_vertion();
    let mut table = table!(
        ["    argument     ", "       description        "],
        ["-----------------", "--------------------------"],
        ["     no args     ", "show stats of current song".blue()],
        ["      play       ", "  play the current song   ".blue()],
        ["      pause      ", "  pause the current song  ".blue()],
        ["      stop       ", "  stop the current song   ".blue()],
        ["      next       ", "    play the next song    ".blue()],
        ["      prev       ", " pause the previous song  ".blue()],
        ["      help       ", "   shows this help menu   ".blue()],
        ["      kill       ", "      kill mpd server     ".blue()],
        ["  --host='HOST'  ", "    host of mpd server    ".blue()],
        ["  --port='PORT'  ", "    port of mpd server    ".blue()],
        [" --silent or -q  ", "      silent output       ".blue()],
        [" --version or -v ", "    print the version     ".blue()],
        ["  --help or -h   ", "      print the help      ".blue()]
    );

    table.set_format(get_format());
    table.printstd();
}

pub fn get_vertion() {
    println!("MPRS version = {}", env!("CARGO_PKG_VERSION").bright_blue());
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
