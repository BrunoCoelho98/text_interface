use std::collections::HashMap;
use std::io;
use std::string::String;

fn main() {
    let mut departments = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command: Vec<_> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

        match &command[0][..] {
            "add" => {
                if let Some(index) = command.iter().position(|s| s == "to") {
                    let name = command[1..index].join(" ");
                    let department = command[index + 1].clone();

                    departments.entry(department).or_insert(Vec::new()).push(name);
                } else {
                    println!("Error: Please specify a department.");
                }
            }
            "list" => {
                if command.len() == 1 {
                    for department in departments.keys() {
                        println!("{}:", department);
                        for name in departments[department].iter() {
                            println!(" {}", name);
                        }
                    }
                } else {
                    let department = command[1].clone();
                    if let Some(names) = departments.get(&department) {
                        println!("People in {} department:", department);
                        for name in names.iter() {
                            println!("  {}", name);
                        }
                    } else {
                        println!("Department {} not found.", department);
                    }
                }
            }
            "quit" => break,
            _ => println!("Unknown command: {}", &command[0]),
        }
    }
}
