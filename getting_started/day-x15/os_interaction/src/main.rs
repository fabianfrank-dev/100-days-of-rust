use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== System Info ===\n");

    print_os_info();
    print_env_info();
    print_uptime();
    print_disk_usage("/");
    print_cpu_count();
}

fn print_os_info() {
    println!("--- OS ---");

    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let family = env::consts::FAMILY;

    println!("  OS:           {}", os);
    println!("  Architecture: {}", arch);
    println!("  Family:       {}", family);

    if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
        println!("  Hostname:     {}", hostname.trim());
    }

    if let Ok(release) = fs::read_to_string("/etc/os-release") {
        for line in release.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                println!("  Distro:       {}", name);
                break;
            }
        }
    }

    println!();
}

fn print_env_info() {
    println!("--- Environment ---");

    let keys = ["USER", "HOME", "SHELL", "TERM", "LANG"];
    for key in &keys {
        match env::var(key) {
            Ok(val) => println!("  {:<8} {}", format!("{}:", key), val),
            Err(_)  => println!("  {:<8} (not set)", format!("{}:", key)),
        }
    }

    println!();
}

fn print_uptime() {
    println!("--- Uptime ---");

    match fs::read_to_string("/proc/uptime") {
        Ok(content) => {
            let secs: f64 = content
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);

            let days    = (secs / 86400.0) as u64;
            let hours   = ((secs % 86400.0) / 3600.0) as u64;
            let minutes = ((secs % 3600.0) / 60.0) as u64;
            let seconds = (secs % 60.0) as u64;

            println!("  {}d {}h {}m {}s", days, hours, minutes, seconds);
        }
        Err(_) => println!("  (unavailable on this platform)"),
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("  Unix time: {}", now);

    println!();
}

fn print_disk_usage(path: &str) {
    println!("--- Disk Usage ({}) ---", path);

    match fs::read_to_string("/proc/mounts") {
        Ok(mounts) => {
            let count = mounts.lines().count();
            println!("  Mounted filesystems: {}", count);
        }
        Err(_) => println!("  (unavailable)"),
    }

    match fs::metadata(path) {
        Ok(meta) => println!("  Root metadata type: {:?}", meta.file_type()),
        Err(e)   => println!("  Could not stat {}: {}", path, e),
    }

    println!();
}

fn print_cpu_count() {
    println!("--- CPU ---");

    match fs::read_to_string("/proc/cpuinfo") {
        Ok(info) => {
            let cores = info.lines().filter(|l| l.starts_with("processor")).count();
            println!("  Logical CPUs: {}", cores);

            if let Some(model_line) = info.lines().find(|l| l.starts_with("model name")) {
                let model = model_line.splitn(2, ':').nth(1).unwrap_or("unknown").trim();
                println!("  Model:        {}", model);
            }
        }
        Err(_) => println!("  (unavailable on this platform)"),
    }

    println!();
}
