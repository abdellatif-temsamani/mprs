use colored::Colorize;

pub fn format_duration(secs: u64) -> String {
    let mins = secs / 60;

    if mins > 60 {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;

        return format!("{}:{}:{}", hours, mins, secs);
    }

    let secs = secs % 60;
    format!("{}:{}", mins, secs)
}

/// same as println! but with **[MPRS]::**
/// for normal prints
/// TODO: to marco
pub fn print(data: String) {
    println!("{} {}", "[MPRS]::".green().bold(), data.green());
}

/// same as eprintln! but with **[MPRS]::**
/// for errors
/// TODO: to marco
pub fn err_print(data: String) {
    println!("{} {}", "[MPRS]::".red().bold(), data.red());
}

/// same as eprintln! but with **[MPRS]::**
/// for warnings
/// TODO: to marco
pub fn warn_print(data: String) {
    println!("{} {}", "[MPRS]::".yellow().bold(), data.yellow());
}
