fn main() {
    // create needed variables
    let log_entry = String::from("[WARN] Disk space reaching 90%");
    let error_code: u32 = 500;
    let mut summary = String::from("System log Summary: \n");

    // extract severity from log entry
    let severity = extract_severity(&log_entry);
    println!("\n The severity of the log entry is: {severity} \n");

    // append an alert to the summary
    append_alert(&mut summary, severity, "Disk space low.");
    println!("{summary}");

    // archive the log entry with the error code
    let archived_log = archive_log(log_entry, error_code);
    println!("Archived log entry: {}", archived_log);
}

// Function to extract severity from a log entry
fn extract_severity(log_entry: &str) -> &str {
    // Check if the log entry contains severity information
    if !log_entry.contains('[') || !log_entry.contains(']') {
        return log_entry;
    } else {
        //Find [ & ]
        let severity_start = log_entry.find('[').unwrap() + 1;
        let severity_end = log_entry.find(']').unwrap();
        return &log_entry[severity_start..severity_end];
    }
}

fn append_alert(summary: &mut String, severity: &str, message: &str) {
    summary.push_str(&format!("[{severity}] {message} \n"));
}

fn archive_log(mut log_entry: String, code: u32) -> String {
    log_entry.push_str(&format!(" [CODE: {code}] \n"));
    log_entry
}
