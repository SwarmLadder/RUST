use swarmladder_rust::summarize;

fn main() {
    let log = "Ownership,25\n\nBorrowing,40";
    match summarize(log) {
        Ok(summary) => println!("{} sessions, {} minutes", summary.sessions, summary.minutes),
        Err((line, reason)) => eprintln!("Line {line}: {reason:?}"),
    }
}
