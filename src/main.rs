use core::panic;
use std::io::{self};
use std::{env, fs};

fn main() {
    // println!("Hello, world!");
    let current_dir = env::current_dir();
    let current_dir = match current_dir {
        Ok(_ans) => _ans,
        Err(_err) => panic!("there is some error {}", _err),
    };

    let mut curr_dir = String::from(current_dir.to_str().unwrap());

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Some Error Occured");

        // io::stdout()
        //     .write("Yoho Yoho".as_bytes())
        //     .expect("Some error Occured");

        let parmas: Vec<&str> = command.trim().split(" ").collect();

        if parmas.len() < 1 || parmas[0] == "" {
            return;
        };

        match parmas[0] {
            "pwd" => {
                println!("{}", curr_dir);
            }
            "ls" => {
                let dir_contents = fs::read_dir(curr_dir.clone());

                let dir_contents = match dir_contents {
                    Ok(_ans) => _ans,
                    Err(_err) => panic!("there is some error {}", _err),
                };
                for content in dir_contents {
                    let name = content.unwrap().file_name();
                    // .to_str(&self) takes pointer to self but returns reference to differnt type str
                    // which is not same as OsString and causes the origial type
                    // thus we need to first store the value given by file_name
                    // or do the below commented out syntax
                    // let name = content.unwrap().file_name().into_string().unwrap();
                    let name = name.to_str().unwrap();
                    println!("{}", name);
                }
            }
            "cd" => {
                let mut flag = false;
                if parmas.len() == 1 {
                    curr_dir = String::from("/");
                    flag = true;
                }
                let mut new_dir = curr_dir.clone();

                if !flag {
                    let paths: Vec<&str> = parmas[1].split("/").collect();
                    for path in paths {
                        if path == ".." {
                            let data: usize = new_dir.rfind('/').unwrap();
                            new_dir = new_dir[0..data].to_string();
                        } else if path == "" {
                            continue;
                        } else {
                            new_dir = new_dir.clone() + "/" + path;
                        }
                    }
                }

                let dir_contents = fs::read_dir(new_dir.clone());

                match dir_contents {
                    Ok(_ans) => {
                        flag = true;
                        curr_dir = new_dir;
                    }
                    Err(_err) => {
                        flag = false;
                        println!("{}", _err)
                    }
                };

                if !flag {
                    println!("No dir found");
                }
            }
            _ => {
                println!("unrecognized command");
                return;
            }
        };
    }
}
