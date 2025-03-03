use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let packages = vec![
        "command-not-found",
        "dbus-python",
        "distro-info",
        "netifaces",
        "PyGObject",
        "python-apt",
        "PyYAML",
        "ubuntu-advantage-tools",
        "ufw",
    ];

    let file_path = Path::new("../../requirements.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut new_contents = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if !packages.iter().any(|pkg| line.contains(pkg)) {
            new_contents.push(line);
        }
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    for line in new_contents {
        writeln!(file, "{}", line)?;
    }
    
    Ok(())
}