use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process::Command;
use std::env;
use std::path;

fn main(){
    let mut input = String::new();
    loop {
        input = shell_readline();
        println!("Input was {}", input);

        //Match input to some program
        if input == "exit"{
            terminate_shell();
        }
        else {
            execute_command(input);
        }
    }
}

fn terminate_shell(){
    std::process::exit(0);
}

fn execute_command(input : String){
    let (jobs, pipecount) = parse_command(&input);
    //Add pipe support
    let (program, arguements) = &jobs[0]; 
    //println!("Program to run {}, arguements {:?}", &program, &arguements);
    if *program == "cd".to_owned(){
        if arguements.len() >= 1{
            match arguements[0].as_ref(){
                "."     => (),
                ".."    => {
                    let mut new_path = env::current_dir().unwrap();
                    new_path.pop();
                    env::set_current_dir(new_path.as_path());
                },
                "-"     => (),
                _       => {
                    let new_path = path::Path::new(&arguements[0]);
                    env::set_current_dir(&new_path).is_ok();
                },
           };
        }
    }

    //Handle gracefully
    else{
        let output = Command::new(program)
                                .args(arguements)
                                .output();
        match output {
            Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
            Err(err) => println!("Cannot find program")
        };
    }
}

fn parse_command(input:&String) -> (Vec<(&str, Vec<&str>)>, usize){
    let temp: Vec<&str> = input.split(" | ").collect();
    let pipe_count = temp.len();
    let mut jobs: Vec<(&str, Vec<&str>)> = Vec::new();
    for elements in temp{
        let mut command_string: Vec<&str> = elements.split(" ").collect();
        let program: &str = command_string.remove(0);
        jobs.push((program, command_string));
    }
    (jobs, pipe_count)
}

fn shell_readline() -> String{
    match env::current_dir() {
        Ok(path) => print!("{}>", path.display()),
        Err(e) => print!("No path found"),
    };
    stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input);
    input.trim().to_string()
}
