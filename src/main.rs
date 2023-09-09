use std::fs;
use std::io;
use std::process::Command;

fn init() -> String {
    let mut storage_path: String = String::from("./storage");

    if fs::metadata(&storage_path).is_ok() {
        println!("The file path '{}' exists.", &storage_path);
    } else {
        println!("The file path '{}' does not exist.", &storage_path);
        println!("Creating the file path '{}'...", storage_path);
        fs::create_dir(&storage_path).expect("Failed to create the file path!");
        println!("Created the file path '{}'!", &storage_path);
    }
    return storage_path
}
fn prompt() -> String {
    println!("===========================");
    println!("Welcome to Homebrew Backup!");
    println!("===========================");
    println!("");
    println!("Commands:");
    println!("backup - Backup your Homebrew packages");
    println!("restore - Restore your Homebrew packages");
    println!("exit - Exit the program");
    println!("settings - Modify the program settings");
    println!("");
    println!("Please enter a command:");
    let mut prompt: String = String::new();
 
    io::stdin().read_line(&mut prompt).expect("Error! Failed to read input.");
    return prompt
}

fn backup(storage_path: String) {
    println!("Backing up your Homebrew packages to {}...", storage_path);
    let output = Command::new("brew")
        .arg("leaves")
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        panic!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let packages = String::from_utf8_lossy(&output.stdout);
    if packages.is_empty() {
        panic!("Command produced no output");
    }
    let path = format!("{}/packages.txt", storage_path);
    fs::write(path, packages.as_bytes()).expect("Failed to write file");
    println!("Backed up your Homebrew packages!");
}


fn restore(storage_path: String) {
    println!("Restoring your Homebrew packages from {}...", storage_path);
    let path = format!("{}/packages.txt", storage_path);
    let binding = fs::read_to_string(path).expect("Failed to read file").replace("\n", " ");
    let packages = binding.split(" ");
    // println!("Packages: {}", packages);
    println!("Please wait... This may take a few minutes...");
;    for package in packages {
        let restore: std::process::Output = Command::new("brew")
        .arg("install")
        .arg(package)
        .output()
        .expect("Failed to execute command");
        // println!("stdout: {}", String::from_utf8_lossy(&restore.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&restore.stderr));
    }
    println!("Successfully restored your Homebrew packages!");
}

fn exit() {

}

fn settings() {

}

fn main() {
    let mut storage_path: String = String::from("");
    let mut storage_path = init();
    let prompt: String = prompt();
    if prompt.contains("backup") {
        backup(storage_path);
    } else if prompt.contains("restore") {
        restore(storage_path);
    } else if prompt.contains("exit") {
        exit();
    } else if prompt.contains("settings") {
        settings();
    } else {
        println!("Invalid command!");   
    }
}