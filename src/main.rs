use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    match execute() {
        Ok(_) => println!("Done"),
        Err(e) => {
            println!("Encountered error:");
            println!("{:#?}", e);
        }
    }
}

fn execute() -> Result<Vec<String>, String> {
    println!("Attempting to run git 'fetch origin --prune'");
    let delete_branches = prune()
        .and_then(|_| get_branches())
        .map(crate::parse_gone)?;

    if delete_branches.is_empty() {
        println!("No branches marked gone");
        return Ok(Vec::new()); // Nothing else to do. Return out.
    }

    let confirmation = user_confirmation(&delete_branches);

    if confirmation.to_uppercase() == "Y" {
        println!(
            "Attempting to delete branches on user confirmation '{}'",
            &confirmation
        );

        return delete_gone_branches(delete_branches);
    } else {
        println!("User did not type Y, not deleting");
    }

    Ok(Vec::new())
}

fn prune() -> Result<(), String> {
    Command::new("git")
        .args(["fetch", "origin", "--prune"])
        .output()
        .map_err(|e| e.to_string())
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .map(|output_str| {
                    println!("Output of prune: {}", &output_str);
                    ()
                })
                .map_err(|e| e.to_string())
        })
}

fn get_branches() -> Result<String, String> {
    println!("Attempting to run 'git branch -v'");
    Command::new("git")
        .args(["branch", "-v"])
        .output()
        .map_err(|e| e.to_string())
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .map(|output_str| {
                    println!("Successfully converted branch output to string");
                    output_str
                })
                .map_err(|e| e.to_string())
        })
}

fn parse_gone(branches: String) -> Vec<String> {
    println!("Sucessfully converted to string");
    println!("Filtering");
    branches
        .to_owned()
        .lines()
        .filter(|line| line.contains("[gone]"))
        .filter(|line| !line.contains('*')) // Skip active branch if it is marked as gone.
        .map(str::trim)
        .map(str::to_string)
        .enumerate()
        .flat_map(|(i, line)| {
            // git branch -v returns more information than just the branch name and [gone].
            // This gets just the branch name or returns None and is filtered out by flat_map
            println!("Parsing line {}: {}", i + 1, &line);
            if let Some(match_location) = line.find(|c: char| c.is_ascii_whitespace()) {
                return line
                    .get(0..match_location)
                    .map(str::trim)
                    .map(str::to_string);
            }

            None
        })
        .filter(|line| !(line == "main" || line == "master")) // Always skip main and master.
        .collect()
}

fn user_confirmation(delete_branches: &Vec<String>) -> String {
    println!("Delete the following branches?");

    delete_branches.iter().enumerate().for_each(|(i, branch)| {
        println!(" {}\t{}", i + 1, branch);
    });

    print!("Y/N: ");
    let _ = stdout().flush();

    let mut user_input = String::new();
    let _input = stdin().read_line(&mut user_input);
    user_input = user_input.trim().to_string();

    println!("User input read: '{}'", &user_input);

    user_input.trim().to_string()
}

fn delete_gone_branches(delete_branches: Vec<String>) -> Result<Vec<String>, String> {
    let delete_output = delete_branches
        .into_iter()
        .map(|branch| {
            Command::new("git")
                .args(["branch", "-D", &branch])
                .output()
                .map_err(|e| e.to_string())
                .and_then(|output| String::from_utf8(output.stdout).map_err(|e| e.to_string()))
        })
        .collect::<Result<Vec<String>, String>>()?;

    delete_output
        .iter()
        .for_each(|output| println!("{}", output));

    Ok(delete_output)
}
