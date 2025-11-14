use dialoguer::{Select, FuzzySelect};
use std::process::{Command, Stdio};

fn main() -> Result<(),String> {
    let initial_options = vec![
        "Install Package",
        "List Packages Installed",
        "Remove Package",
        "Update Packages",
        "Exit"
    ];
    'lop: loop {
        clear_screen();
        let options = Select::new()
            .with_prompt("Select one of the options:")
            .items(&initial_options)
            .interact();
        match options {
            Ok(option) => {
                match initial_options[option] {
                    "Install Package" => install_packages(),
                    "List Packages Installed" => print_packages_installed(),
                    "Remove Package" => remove_package(),
                    "Update Packages" => update_packages(),
                    "Exit" => break 'lop,
                    _ => break 'lop,    // Default in case of error
                }
            },
            Err(_) => return Err("Error not expected, closing program.".to_string()),
        }
    };
    Ok(())
}

// Type to save individual packages
type Package = String;

// Two simple functions to list packages, no much to say
// 0 for installed, 1 for available,
fn list_packages_options(ind: usize) -> Vec<Package> {
    let list_packages = |option: &str| -> Vec<Package> {
        let out = Command::new("dnf")
            .args(["list",option])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let text = String::from_utf8(out.stdout).unwrap();
        text
            .lines()
            .skip(1)
            .map(
                |s| {
                    s.split(' ').collect::<Vec<_>>().first().unwrap().to_string()
                }
            )
            .filter(
                |s|
                    !s.ends_with(".i686")
            )
            .collect()
    };
    list_packages(
        match ind {
            0 => "--installed",
            1 => "--available",
            _ => "",
        }
    )
}

// Installation of packages
fn install_packages() {
    let packages = list_packages_options(1);
    let mut cpy = packages.clone();
    cpy.push("Exit".to_string());
    let opt = FuzzySelect::new()
        .with_prompt("Select package to install (Select Exit to go back):")
        .items(cpy)
        .interact();
    match opt {
        Ok(option) => {
            let n = packages.len();
            if option < n {
                let end = Command::new("sudo")
                    .args(["dnf","install",packages[option].as_str(),"-y"])
                    .status();
                if end.is_ok() {
                    println!("Successfully installed package: {}",packages[option]);
                }
                else {
                    println!("Error installing the package.");
                }
            }
        },
        Err(_) => println!("Unexpected error."),
    };
}

// Function only to print all the packages installed
fn print_packages_installed() {
    let packages = list_packages_options(0);
    let _ = FuzzySelect::new()
        .with_prompt("Select any package to go back to the menu: ")
        .items(packages)
        .interact();
}

// Function to remove a package, with a search option to filter the package
fn remove_package() {
    let packages = list_packages_options(0);
    let mut cpy = packages.clone();
    cpy.push("Exit".to_string());
    let opt = FuzzySelect::new()
        .with_prompt("Select package to remove (WARNING: It may break your system if it's a dependency.) (Select Exit to go back):")
        .items(cpy)
        .interact();
    match opt {
        Ok(option) => {
            let n = packages.len();
            if option < n {
                let end = Command::new("sudo")
                    .args(["dnf","remove",packages[option].as_str(),"-y"])
                    .status();
                if end.is_ok() {
                    println!("Successfully removed package: {}",packages[option]);
                }
                else {
                    println!("Error removing the package.");
                }
            }
        },
        Err(_) => println!("Unexpected error."),
    }
}

// Function to update packages, only for clean code
fn update_packages() {
    Command::new("sudo")
        .args(["dnf","update","-y"])
        .status()
        .expect("Error with the update.");
}

fn clear_screen() {
    Command::new("clear").status().expect("I dont know how a clear failed");
}