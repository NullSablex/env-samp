fn main() {
    let output = std::process::Command::new("date")
        .arg("+%b %d %Y|%H:%M:%S|%Y")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok());

    let (date, time, year) = output.map_or_else(
        || (unknown(), unknown(), unknown()),
        |raw| {
            let trimmed = raw.trim();
            let parts: Vec<&str> = trimmed.splitn(3, '|').collect();
            (
                parts.first().copied().unwrap_or("Unknown").to_string(),
                parts.get(1).copied().unwrap_or("Unknown").to_string(),
                parts.get(2).copied().unwrap_or("Unknown").to_string(),
            )
        },
    );

    println!("cargo:rustc-env=BUILD_DATE={date}");
    println!("cargo:rustc-env=BUILD_TIME={time}");
    println!("cargo:rustc-env=BUILD_YEAR={year}");

    generate_inc();
}

fn unknown() -> String {
    "Unknown".to_string()
}

fn generate_inc() {
    use std::fs;

    let template_path = "include/env_samp.inc.in";
    let output_path = "include/env_samp.inc";

    let template = fs::read_to_string(template_path)
        .unwrap_or_else(|e| panic!("failed to read {template_path}: {e}"));

    let version = env!("CARGO_PKG_VERSION");
    let rendered = template.replace("{{VERSION}}", version);

    if fs::read_to_string(output_path).ok().as_deref() != Some(rendered.as_str()) {
        fs::write(output_path, &rendered)
            .unwrap_or_else(|e| panic!("failed to write {output_path}: {e}"));
    }
}
