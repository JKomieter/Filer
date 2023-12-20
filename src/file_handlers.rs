use colored::*;
use difference::{Changeset, Difference};
use std::{fs, process};

pub fn handle_cmd(cmd: &str, filename: &str) -> Result<(), &'static str> {
    match cmd {
        "create" => {
            create_file(&filename)?;
        }
        "delete" => {
            delete_file(&filename)?;
        }
        "help" => {
            help();
        }
        "version" => {
            create_file_version(&filename)?;
        }
        "cmp" => {
            compare_versions(&filename)?;
        }
        _ => {
            println!("Enter help flag for usage information");
            return Err("Command not found");
        }
    };

    Ok(())
}

pub fn read_file_contents(filename: &str) -> Result<String, &'static str> {
    let filename = if filename.ends_with(".txt") {
        filename.to_string()
    } else {
        format!("{}.txt", filename)
    };

    let contents = fs::read_to_string(&filename).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    Ok(contents)
}


pub fn create_file(filename: &str) -> Result<(), &'static str> {
    let filename = if filename.ends_with(".txt") {
        filename.to_string()
    } else {
        format!("{}.txt", filename)
    };

    match fs::File::create(&filename) {
        Ok(file) => {
            println!("Created file: {}", filename);
            file
        }
        Err(_) => return Err("Could not create file"),
    };

    Ok(())
}

pub fn create_file_version(filename: &str) -> Result<(), &'static str> {
    let contents = read_file_contents(&filename)?;

    let new_filename = format!("{}_v2.txt", filename);

    match fs::write(&new_filename, contents) {
        Ok(_) => {
            println!("Created file: {}", new_filename);
        }
        Err(_) => return Err("Could not create file"),
    };

    Ok(())
}

pub fn delete_file(filename: &str) -> Result<(), &'static str> {
    let filename = if filename.ends_with(".txt") {
        filename.to_string()
    } else {
        format!("{}.txt", filename)
    };

    match fs::remove_file(&filename) {
        Ok(_) => {
            println!("Deleted file: {}", filename);
        }
        Err(_) => return Err("Could not delete file"),
    };

    Ok(())
}

pub fn compare_versions(filename: &str) -> Result<(), &'static str> {
    let old_contents = read_file_contents(&filename)?;

    let new_contents = read_file_contents(&format!("{}_v2", filename))?;

    let changeset = Changeset::new(&old_contents, &new_contents, "\n");

    for diff in &changeset.diffs {
        match diff {
            Difference::Same(ref x) => {
                println!("{}", x)
            }
            Difference::Add(ref x) => {
                for line in x.split("\n") {
                    let old_line = changeset
                        .diffs
                        .iter()
                        .filter_map(|diff| match diff {
                            Difference::Same(y) | Difference::Rem(y) => y.split("\n").next(),
                            _ => None,
                        })
                        .find(|y| line.contains(y));

                    if let Some(old_line) = old_line {
                        let old_words: Vec<_> = old_line.split_whitespace().collect();
                        let new_words: Vec<_> = line.split_whitespace().collect();

                        for word in new_words {
                            if old_words.contains(&word) {
                                println!("{} ", word);
                            } else {
                                println!("+ {} ", word.green());
                            }
                        }
                        println!();
                    } else {
                        println!("+ {}", line.green());
                    }
                }
            }
            Difference::Rem(ref x) => {
                for line in x.split("\n") {
                    let new_line = changeset
                        .diffs
                        .iter()
                        .filter_map(|diff| match diff {
                            Difference::Same(y) | Difference::Add(y) => y.split("\n").next(),
                            _ => None,
                        })
                        .find(|y| line.contains(y));

                    if let Some(new_line) = new_line {
                        let new_words: Vec<_> = new_line.split_whitespace().collect();
                        let old_words: Vec<_> = line.split_whitespace().collect();

                        for word in old_words {
                            if new_words.contains(&word) {
                                println!("{} ", word);
                            } else {
                                println!("- {} ", word.red());
                            }
                        }
                        println!();
                    } else {
                        println!("-{}", line.red());
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn help() {
    println!("Usage: cargo run [command] [filename]");
    println!("Commands:");
    println!("  create: Create a new file");
    println!("  delete: Delete a file");
    println!("  help: Display this message");
    process::exit(1);
}
