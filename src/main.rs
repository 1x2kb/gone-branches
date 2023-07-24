use std::{io::stdin, process::Command};

fn main() {
    match run() {
        Ok(_) => println!("Done"),
        Err(e) => {
            println!("Encountered error:");
            println!("{:#?}", e);
        }
    }
}

// TODO: Refactor... Lots of code here that can be easily chunked up.
fn run() -> Result<(), String> {
    println!("Attempting to run git 'fetch origin --prune'");
    let delete_branches = Command::new("git")
        .args(["fetch", "origin", "--prune"])
        .output()
        .map_err(|e| e.to_string())
        .and_then(|_| {
            println!("Successfully ran 'fetch origin --prune'");
            println!("Attempting to run 'git branch -v'");
            Command::new("git")
                .args(["branch", "-v"])
                .output()
                .map_err(|e| e.to_string())
        })
        .and_then(|output| {
            println!("Successfully ran 'git branch -v'");
            println!("Attempting to convert to string");
            String::from_utf8(output.stdout).map_err(|e| e.to_string())
        })
        .map(|lines| {
            println!("Sucessfully converted to string");
            println!("Filtering");
            lines
                .lines()
                .filter(|line| line.contains("[gone]"))
                .filter(|line| !line.contains('*')) // Skip active branch if it is marked as gone.
                .map(str::trim)
                .map(str::to_string)
                .flat_map(|line| {
                    // git branch -v returns more information than just the branch name and [gone].
                    // This gets just the branch name or returns None and is filtered out by flat_map
                    println!("Parsing line: {}", &line);
                    if let Some(match_location) = line.find(|c: char| c.is_ascii_whitespace()) {
                        return line.get(0..match_location).map(str::to_string);
                    }

                    None
                })
                .filter(|line| !(line == "main" || line == "master")) // Always skip main and master.
                .fold(String::new(), |aggregator, line| aggregator + " " + &line)
                .trim()
                .to_string()
        })?;

    if delete_branches.is_empty() {
        println!("No branches marked gone");
        return Ok(()); // Nothing else to do. Return out.
    }

    println!("Delete the following branches? Y/N: {}", &delete_branches);
    let mut user_input = String::new();
    let _input = stdin().read_line(&mut user_input);
    let user_input = user_input.trim().to_string();

    if user_input.to_uppercase() == "Y" {
        println!(
            "Attempting to delete branches on user confirmation '{}'",
            &user_input
        );

        delete_branches
            .split(' ')
            .map(|branch| {
                Command::new("git")
                    .args(["branch", "-D", branch])
                    .output()
                    .map_err(|e| e.to_string())
                    .and_then(|output| String::from_utf8(output.stdout).map_err(|e| e.to_string()))
            })
            .collect::<Result<Vec<String>, String>>()?
            .iter()
            .for_each(|output| println!("{}", output));
    } else {
        println!("User did not type Y, not deleting");
        println!("User input read: '{}'", &user_input);
    }

    Ok(())
}
