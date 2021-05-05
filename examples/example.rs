#[macro_use]
extern crate manifest_dir_macros;

fn main() {
    println!(path!("Cargo.toml"));
    println!(path!("src/lib.rs"));
    println!(path!("src", "lib.rs"));
    println!(path!("src", "lib.rs", "/bin"));
    println!(path!("/usr"));

    println!(exist_path!("Cargo.toml"));
    println!(directory_path!("src"));
    println!(not_directory_path!("Cargo.toml"));
    println!(file_path!("Cargo.toml"));

    println!(relative_path!("Cargo.toml"));
    println!(directory_relative_path!("src"));
    println!(not_directory_relative_path!("Cargo.toml"));
    println!(file_relative_path!("Cargo.toml"));
}
