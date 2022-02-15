use std::fs::remove_dir_all;
use std::path::Path;
use std::process::{exit, Command};

macro_rules! execute {
    ($command:expr) => {
        let output = match $command.output() {
            Ok(value) => value,
            Err(error) => {
                println!(
                    "failed to execute process: {}",
                    $command.get_program().to_string_lossy()
                );
                println!("{error}");
                exit(1);
            }
        };
        if !output.status.success() {
            println!("process exited with status code: {}", output.status);
            if output.stdout.len() == 0 {
                println!("-- {} stdout:", $command.get_program().to_string_lossy());
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            if output.stderr.len() == 0 {
                println!("-- {} stderr:", $command.get_program().to_string_lossy());
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            exit(1);
        }
    };
}

const XI_REPO_URL: &str = "https://github.com/xi-editor/xi-editor.git";

fn main() {
    println!("The output directory is: {}", &out_dir);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let clone_path = Path::new(&out_dir).join("xi-repo");

    if clone_path.exists() && clone_path.read_dir().unwrap().next().is_some() {
        remove_dir_all(clone_path.clone());
    }

    execute!(Command::new("git")
        .arg("clone")
        .args(["--depth", "1"])
        .arg(XI_REPO_URL)
        .arg(clone_path.to_string_lossy()));

    let cargo_toml_path = Path::new(&out_dir).join("xi-repo").join("Cargo.toml");
    execute!(Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg(format!(
            "--manifest-path={}",
            cargo_toml_path.to_string_lossy()
        )));
}
