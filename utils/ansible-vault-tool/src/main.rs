use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};

fn main() {
    println!("[DEBUG] Starting program");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("{} decrypt <file>", args[0]);
        println!("{} encrypt <file>", args[0]);
        println!("{} encrypt-all", args[0]);
        return;
    }
    
    let command = &args[1];
    println!("[DEBUG] Command: {}", command);
    
    match command.as_str() {
        "decrypt" => {
            if args.len() < 3 {
                eprintln!("Missing file path for decryption");
                return;
            }
            let vault_password = read_vault_password("ansible_vault_pass.raw");
            decrypt_file(&args[2], &vault_password);
        }
        "encrypt" => {
            if args.len() < 3 {
                eprintln!("Missing file path for encryption");
                return;
            }
            let vault_password = read_vault_password("ansible_vault_pass.raw");
            encrypt_file(&args[2], &vault_password);
        }
        "encrypt-all" => {
            println!("[DEBUG] Encrypting all .yml.raw files in playbooks/files using ansible_vault_pass.raw");
            let vault_password = read_vault_password("ansible_vault_pass.raw");
            encrypt_all_files("playbooks/files", &vault_password);
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn decrypt_file(file_path: &str, _vault_password: &str) {
    if !file_path.ends_with(".enc") {
        eprintln!("Invalid file format for decryption: {}", file_path);
        return;
    }
    let raw_file = PathBuf::from(file_path.trim_end_matches(".enc")).with_extension("raw");
    println!("[DEBUG] Decrypting: {} -> {}", file_path, raw_file.display());
    let output = Command::new("ansible-vault")
        .args(["decrypt", file_path, "--output", raw_file.to_str().unwrap(), "--vault-password-file", "ansible_vault_pass.raw"])
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to decrypt file");
    
    if !output.status.success() {
        eprintln!("Decryption failed: {}", file_path);
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn encrypt_file(file_path: &str, _vault_password: &str) {
    if !file_path.ends_with(".raw") {
        eprintln!("Invalid file format for encryption: {}", file_path);
        return;
    }
    let enc_file = PathBuf::from(file_path.trim_end_matches(".raw")).with_extension("yml.enc");
    println!("[DEBUG] Encrypting: {} -> {}", file_path, enc_file.display());
    
    let output = Command::new("ansible-vault")
        .args(["encrypt", file_path, "--output", enc_file.to_str().unwrap(), "--vault-password-file", "ansible_vault_pass.raw"])
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to encrypt file");
    
    if !output.status.success() {
        eprintln!("Encryption failed: {}", file_path);
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn encrypt_all_files(dir: &str, vault_password: &str) {
    println!("[DEBUG] Searching for .yml.raw files in: {}", dir);
    let files = find_files(dir);
    println!("[DEBUG] Found {} files to encrypt", files.len());
    for file in &files {
        println!("[DEBUG] Encrypting file: {}", file);
        encrypt_file(file, vault_password);
    }
}

fn find_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_files(path.to_str().unwrap()));
            } else if let Some(file_name) = path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    if file_str.ends_with(".yml.raw") {
                        println!("[DEBUG] Found matching file: {}", path.display());
                        files.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    files
}

fn read_vault_password(file_path: &str) -> String {
    if let Ok(mut file) = File::open(file_path) {
        let mut password = String::new();
        file.read_to_string(&mut password).expect("Failed to read vault password file");
        return password.trim().to_string();
    }
    
    print!("Enter Vault Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password input");
    
    let trimmed_password = password.trim().to_string();
    
    if let Ok(mut file) = File::create(file_path) {
        writeln!(file, "{}", trimmed_password).expect("Failed to write vault password file");
    }
    
    trimmed_password
    
    print!("Enter Vault Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password input");
    password.trim().to_string()
}
