use colored::Colorize;

pub fn run() {
    println!("{}", "Arona Version Information".bright_blue().bold());
    println!();
    println!("Arona: {}", env!("CARGO_PKG_VERSION"));
    println!("BlueArch: 0.1-dev");
    println!("Build: development");
}
