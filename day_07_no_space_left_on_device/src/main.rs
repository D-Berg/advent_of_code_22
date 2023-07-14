// filesystem is a hashmap of directorys.
//

use std::{io::{BufRead, BufReader}, collections::HashMap};

type DirectoryIndex = usize;

#[derive(Debug)]
struct FileSystem {
    directories: Vec<Directory>,
    current_directory: DirectoryIndex,
}


impl FileSystem {
    /// create a new filesystem 
    fn new() -> FileSystem {

        FileSystem{
            directories: Vec::new(),
            current_directory: 0,
        }
    }

    fn create_dir(&mut self, name: String) {

        self.directories.push(Directory::new_root_dir(name));

        self.current_directory = self.directories.len() - 1;

    }

    /// Creates a file in current directorie
    fn create_file(&mut self, file_name: String, file_size: i32) {
        
        let file = File::new(file_name, file_size);

        self.directories[self.current_directory].files.push(file);

    }

    fn create_sub_dir(&mut self, name: String) {

        self.directories.push(Directory::new_sub_dir(self.current_directory, name.clone()));

        let sub_dir = self.directories.len() - 1;

        self.directories[self.current_directory].sub_directories.insert(name, sub_dir);

    }

    fn cd_parent(&mut self) {
        self.current_directory = self.directories[self.current_directory]
            .parent_directory
            .unwrap()
            .clone();
    }

    fn cd_sub(&mut self, dir_name: String) {

        self.current_directory = self.directories[self.current_directory]
            .sub_directories
            .get(&dir_name)
            .unwrap()
            .clone();
    }

    /// calculate and fill each directories size field.
    /// visit each sub, check if sub_directories is empty 
    /// we know we havnt visited a dir if size = None
    /// DFS
    fn calculate_dir_sizes(&mut self) {

        let mut u: DirectoryIndex = 0;

        let mut stack: Vec<DirectoryIndex> = Vec::new();

        let _n_directories = &self.directories.len();


        // stack of directories to visit 
        stack.push(u);

        // 1. Check if sub_directories are is_empty
        // 2.a) if empty: calculate sum of filesizes
        // 2.b) if not empty: put u on stack and then put sub_directories on stack.
        while !stack.is_empty() {


            u = stack.pop().expect("Failed to pop stack");

            
            let u_sub_directories = &self.directories[u].sub_directories;

            match u_sub_directories.is_empty() {

                true => { 

                    // calculate directory size by summing size of files

                    let mut size = 0;

                    for file in self.directories[u].files.iter() {
                        size += file.size.clone();
                    }
                    
                    // set size of dir to total size of files
                    
                    self.directories[u].size = Some(size);
                }, 

                false => {



                    // check if all direct subs sizes are known.

                    let mut all_subs_sizes_are_known: bool = true;
                    for (_sub_name, u_sub) in u_sub_directories.iter() {

                        if self.directories[u_sub.clone()].size == None {

                            all_subs_sizes_are_known = false;

                        }
                        
                    }

                    match all_subs_sizes_are_known {
                        true => {

                            let mut size = 0;
                            // calculate size of u = size of files + size of subs

                            for file in self.directories[u].files.iter() {

                                size += file.size;

                            }


                            for (_sub_name, u_sub) in u_sub_directories.iter() {
                                size += self.directories[u_sub.clone()]
                                    .size
                                    .expect("Apparently sub_dir size was not know.");
                            }

                            self.directories[u].size = Some(size);
                        },

                        false => {
                            // Push u back on stack

                            stack.push(u);
                            
                            // TODO: Loop through sub directories and push unknown to stack
                            

                            for (_sub_name, u_sub) in u_sub_directories.iter() {
                                if self.directories[u_sub.clone()].size == None {
                                    stack.push(u_sub.clone());
                                }
                            }
 
                        }
                    }


                }

            }

        }



        // for dir_index in 0..n_directories {
        //     
        //     let mut size = 0;
        //
        //     // calculate size of files
        //
        //     for file in self.directories[dir_index].files.iter() {
        //
        //         size += file.size;
        //     }
        //
        //     // calculate size of sub_directories
        //     for (dir_name, sub_idx) in self.directories[dir_index].sub_directories.iter() {
        //
        //         size += self.directories[sub_idx.clone()].size.expect("folder_size not calculated");
        //
        //     }
        //     self.directories[dir_index].size = Some(size);
        //
        //
        //
        // }

    }

}

#[derive(Debug)]
struct Directory {
    name: String,
    parent_directory: Option<DirectoryIndex>,
    sub_directories: HashMap<String, DirectoryIndex>,
    files: Vec<File>,
    size: Option<i32>,
}


impl Directory {
    
    fn new_root_dir(name: String) -> Directory {

        Directory { 
            name: name, 
            parent_directory: None,
            sub_directories: HashMap::new(), 
            files: Vec::new(),
            size: None,
        }

    }

    fn new_sub_dir(parent: DirectoryIndex, name: String) -> Directory {

        Directory {
            name: name,
            parent_directory: Some(parent),
            sub_directories: HashMap::new(),
            files: Vec::new(),
            size: None,
        }
    }

}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: String, size: i32) -> File {

        File {
            name,
            size
        }
    }
}

fn main() {

    // create a root dir
    //

    let mut file_system = FileSystem::new();

    let path = "input.txt";

    let file = std::fs::File::open(path)
        .expect("Failed to open file");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line = lines
        .next()
        .unwrap()
        .expect("errr"); 

    let first_command: Vec<&str> = line
        .split_whitespace()
        .collect();


    file_system.create_dir(first_command[2].to_string());


    while let Some(line) = lines.next() {

        let line_string = line
            .expect("Failed to read line");

        let split_str = line_string
            .split_whitespace()
            .collect::<Vec<&str>>();

        match split_str[0] {

            "$" => { // a command
                
                match split_str[1] {
                    "cd" => {

                        match split_str[2] {
                            ".." => {
                                file_system.cd_parent();
                            }, 

                            _ => {
                                let name = split_str[2].to_string();
                                file_system.cd_sub(name);

                            }
                        }


                    },

                    _ => {

                    }
            
                }

            }, 

            "dir" => {

                let name = split_str[1].to_string();
                file_system.create_sub_dir(name);

            }

            _ => {

                //file 
                
                let file_size = split_str[0]
                    .parse::<i32>()
                    .expect("Failed to parse");

                let file_name = split_str[1].to_string();

                file_system.create_file(file_name, file_size);

            }

        }
        

    }

    file_system.calculate_dir_sizes();

    let mut total_size = 0;
    
    for dir in file_system.directories.iter() {
        let dir_size = dir.size.unwrap();
        if dir_size < 100000 {
            total_size += dir_size;

        }
    }

    dbg!(&file_system.directories);

    println!("Total size of directories less than 100 000: {} bytes", total_size);
}
