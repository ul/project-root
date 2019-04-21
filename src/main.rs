use clap::{crate_version, App};
use std::path::Path;
use std::process::exit;

fn main() {
    let mut pwd = std::env::current_dir().unwrap();

    let matches = App::new("project-root")
        .version(crate_version!())
        .author("Ruslan Prokopchuk <fer.obbee@gmail.com>")
        .about("goto file")
        .args_from_usage("<NAME>... 'Project root markers in priority order (i.e. lerna.json package.json .git )'")
        .get_matches();

    let names: Vec<&Path> = matches
        .values_of("NAME")
        .unwrap()
        .map(|x| Path::new(x))
        .collect();

    let mut roots: Vec<Option<String>> = Vec::new();
    for _ in &names {
        roots.push(None)
    }

    loop {
        for (i, name) in names.iter().enumerate() {
            if roots[i] == None && pwd.join(name).exists() {
                roots[i] = Some(pwd.to_str().unwrap().to_string())
            }
        }
        if !pwd.pop() {
            break;
        }
    }
    for root in roots {
        if let Some(root) = root {
            print!("{}", root);
            exit(0);
        }
    }
    exit(1)
}
