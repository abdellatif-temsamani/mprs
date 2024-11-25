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
