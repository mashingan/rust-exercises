use std::collections::HashMap;
use std::io;

fn main() {
    let mut employee: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        //print!(">>");
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();
        let cmds: Vec<&str> = command.split(' ').collect();
        if cmds.len() < 1 {
            print_help();
            continue;
        }
        let cmd = &cmds[0].to_lowercase();
        if cmd == "help" {
            print_help();
        } else if cmd == "list" {
            //println!("to list");
            if cmds.len() != 2 {
                list_all_employee(&employee);
            } else {
                let dept = &cmds[1];
                list_employee_in(dept, &employee);
            }
        } else if cmd == "add" {
            //println!("to add");
            if cmds.len() != 4 {
                println!("Please use add command correctly: \
add <Name> to <dept>");
                continue;
            }
            let name = &cmds[1];
            let dept = &cmds[3];
            //employee.insert(name.to_string(), dept.to_string());
            let dept = dept.to_string();
            let name = name.to_string();
            println!("{} is going to be added to {}", name, dept);
            match employee.get_mut(&dept) {
                Some(members) => members.push(name),
                None => {
                    let newlist = vec![name];
                    let _ = employee.insert(dept, newlist);
                }
            }
            println!("success");
        } else if cmd == "quit" {
            println!("Thank you.\nGood bye");
            break;
        } else {
            print_help();
        }
    }
}

fn list_employee_in(dept: &str, list: &HashMap<String, Vec<String>>) {
    match list.get(dept) {
        Some(members) => {
            for emp in members {
                println!("{}", emp);
            }
        },
        None => {}
    }
}

fn list_all_employee(list: &HashMap<String, Vec<String>>) {
    for (dept, members) in list.iter() {
        println!("Employee in {}:", dept);
        for member in members {
            println!("{}", member);
        }
        println!("------------------");
    }
}

fn print_help() {
    let info = r"Available command:
1. [Add|add] <name> to <dept>
  To add the <name> of employee to <dept>.
  If the employee already available in that <dept>, it will update instead.
  e.g.:
  add Mashingan to research

2. [List|list] [<dept>|all|]
  List the employee available in given <dept>, if the argument is all or
  nothing, it will list all available employee in all depts.
  e.g.:
  list research
  >>Mashingan in research
3. [Quit|quit|q|Exit|exit]
  To quit the program.";
    println!("{}", info);
}
