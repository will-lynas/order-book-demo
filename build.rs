use std::process::Command;

fn main() {
    let status = Command::new("npx")
        .args([
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "index.css",
            "-o",
            "static/index.css",
            "--minify",
        ])
        .status()
        .expect("Failed to run tailwindcss command");

    if !status.success() {
        panic!("tailwindcss command failed with status: {:?}", status);
    }

    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=index.css");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=src");
}
