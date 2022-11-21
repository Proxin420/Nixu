extern crate rustyline;
mod core;

use std::env;
use std::path;
use std::process::Command;
use std::process::Stdio;
use std::process;
use std::collections::HashMap;
use std::collections::VecDeque;
use rustyline::Editor;
use rustyline::error::ReadlineError;

struct Shell {
    rustyline: Editor<()>,
    history: VecDeque<String>,
    prompt: String,
    aliases: HashMap<String, String>,
}

impl Shell {
    fn new(new_prompt: String, rl: Editor<()>, alias: HashMap<String, String>) -> Shell {
        return Shell {
            rustyline: rl,
            history: VecDeque::new(),
            prompt: new_prompt,
            aliases: alias,
        }
    }

    fn exec(&self, command: Vec<&str>) {
        let (cmd, args) = self.parse(command);
        match cmd.as_str() {
            "" => {
                /* DO NOTHING */
            },
            "cd" => {
                if args.len() > 1 {
                    println!("nixu: Expected 1 argument");
                }
                self.change_directory(&args[0]);
            },
            "exit" => {
                process::exit(0);
            },
            "history" => {
                for value in &self.history {
                    println!("{}", value);
                }
            },
            _ => {
                let result = Command::new(cmd)
                    .args(args)
                    .stdout(Stdio::inherit())
                    .stdin(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn();
                if result.is_err() {
                    println!("nixu: {}", result.unwrap_err());
                } else {
                    result.unwrap().wait().unwrap();
                }
            },
        }
    }

    fn parse(&self, arguments: Vec<&str>) -> (String, Vec<String>) {
        let mut cmd = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut ctr = 0;
        for argument in arguments {
            if ctr == 0 {
                cmd = String::from(argument);
            } else {
                args.push(String::from(argument));
            }
            ctr += 1;
        }
        return (cmd, args);
    }

    fn input(&mut self) -> String {
        let input = self.rustyline.readline(&self.prompt);
        match input {
            Ok(line) => {
                return line;
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                return String::new();
            },
            _ => {
                return String::new();
            },
        }
    }

    fn alias(&mut self, cmd: String, history_cap: &usize) -> String {
        if self.history.len() > *history_cap {
            self.history.pop_front();
        }
        self.history.push_back(cmd.clone());
        let value = self.aliases.get(&cmd);
        if value.is_none() {
            return cmd;
        } else {
            return value.unwrap().to_string();
        }
    }

    fn change_directory(&self, path: &str) {
        let result = env::set_current_dir(path::Path::new(path));
        if result.is_err() {
            println!("nixu: {}", result.unwrap_err());
        }
    }
}

fn main() {
    let rustyline = rustyline::Editor::<()>::new().unwrap();
    let mut config = core::config::Config::new();
    let mut shell = Shell::new(config.prompt.clone(), rustyline, config.aliases);
    for command in config.startup {
        shell.exec(
            command
            .split(" ")
            .collect::<Vec<&str>>()
        );
    }
    loop {
        config = core::config::Config::new();
        shell.prompt = config.prompt;
        let mut input = shell.input();
        input = shell.alias(input, &config.history_cap);
        shell.exec(
                   input
                   .split(" ")
                   .collect::<Vec<&str>>()
        );
    }
}


