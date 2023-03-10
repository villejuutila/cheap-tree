use std::io::Write;
use clap::Parser;
use tree::Generator;

mod config;
mod tree;

fn main() {    
    let args = config::Args::parse();
    
    let target_dir = match &args.dir {
        Some(dir) => {
            if dir.eq(&String::from(".")) {
                std::env::current_dir().expect("Couldn't get current working dir!").as_path().to_owned()
            } else {
                std::path::Path::new(dir).to_owned()
            }
        },
        None => std::env::current_dir().expect("Couldn't get current working dir!").as_path().to_owned()
    };
    
    if !target_dir.exists() {
        println!("Invalid target directory!");
        return;
    }
    
    let max_depth = match &args.max_depth {
        Some(depth) => depth,
        None => &1
    };
    
    let exclude_dirs = match &args.exclude {
        Some(dirs) => dirs.to_owned(),
        None => vec![]
    };
    
    let filter_extensions = match &args.filter_extension {
        Some(extensions) => extensions.to_owned(),
        None => vec![]
    };
    
    let output_to_file = match &args.output {
        Some(_) => true,
        None => false
    };
    let output_file = match output_to_file {
        true => {
            Some(std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(args.output.unwrap()))
        },
         false => None
    };
    
    let mut directory_tree = Generator::init(
        target_dir,
        args.dir_only,
        exclude_dirs,
        args.include_hidden,
        args.colored,
        max_depth.to_owned(),
        filter_extensions
    );
    let tree = directory_tree.build_tree();
    
    for branch in tree {
        println!("{}", branch);
        if output_to_file {
            match &output_file.as_ref().unwrap() {
                Result::Ok(file) => {
                    let mut file = file;
                    let line_to_write = format!("{branch}\n");
                    file.write(&strip_ansi_escapes::strip(line_to_write).unwrap()).expect("Couldn't write to file.");
                },
                Result::Err(error) => panic!("{error}")
            }
        }
    }
}