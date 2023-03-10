use std::{path::PathBuf, fs::{DirEntry}};
use colored::Colorize;

const ELBOW: &str = "└───";
const TEE: &str = "├───";
const PIPE_PREFIX: &str = "│    ";
const SPACE_PREFIX: &str = "     ";

pub struct Generator {
  root_dir: PathBuf,
  dir_only: bool,
  include_hidden: bool,
  colored: bool,
  exclude_dirs: Vec<String>,
  filter_extensions: Vec<String>,
  max_depth: u16,
  tree: Vec<String>,
  os_separator: String,
  total_directories: u32,
  total_files: u32
}

impl Generator {
    pub fn init(
      root_dir: PathBuf,
      dir_only: bool,
      exclude_dirs: Vec<String>,
      include_hidden: bool,
      colored: bool,
      max_depth: u16,
      filter_extensions: Vec<String>
    ) -> Self {
    let os_separator: String = match std::env::consts::OS {
      "windows" => String::from("\\"),
      _ => String::from("/")
    };
    
    Generator {
      root_dir,
      dir_only,
      include_hidden,
      colored,
      exclude_dirs,
      filter_extensions,
      max_depth,
      os_separator,
      tree: vec![],
      total_directories: 0,
      total_files: 0
    }
  }
  
  pub fn build_tree(&mut self) -> &Vec<String> {
    self.build_tree_head();
    self.build_tree_body(self.root_dir.clone(), &String::new(), 0);
    self.tree.push(format!("\n{} directories, {} files", &self.total_directories, &self.total_files));
    
    return &self.tree;
  }
  
  fn build_tree_head(&mut self) {
    let mut root_dir_str = self.root_dir.to_str().unwrap().to_string();
    if !root_dir_str.ends_with(&self.os_separator) {
      root_dir_str = format!("{root_dir_str}{}", self.os_separator);
    }
    self.add_line(true, format!("{}", root_dir_str));
  }
  
  fn build_tree_body(&mut self, dir: PathBuf, prefix: &str, depth: u16) {
    if depth == self.max_depth {
      return;
    }
    let directory_entries = match std::fs::read_dir(dir) {
      Result::Ok(entries) => entries,
      Result::Err(error) => panic!("Couldn't read directory content: {error}")
    };
    let mut directory_content: Vec<DirEntry> = directory_entries
      .into_iter()
      .map(|dir| {
        dir.expect("Couldn't read file")
      })
      .filter(|dir| self.exclude_dir(dir))
      .filter(|dir| self.filter_extension(dir))
      .collect();
    directory_content.sort_by(|a, b| b.file_type().unwrap().is_dir().cmp(&a.file_type().unwrap().is_dir()));
    
    if self.dir_only {
      directory_content = directory_content.into_iter().filter(|dir| dir.file_type().unwrap().is_dir()).collect();
    }
    
    if !self.include_hidden {
      directory_content = directory_content.into_iter().filter(|dir| !dir.file_name().to_str().unwrap().starts_with(".")).collect();
    }
    
    let directories_count = directory_content.len();
    for (pos, entry) in directory_content.iter().enumerate() {
      let connector = if pos == (directory_content.len() - 1) {
        ELBOW
      } else {
        TEE
      };
      if entry.file_type().unwrap().is_dir() {
        self.add_directory(
              entry,
              &pos,
              &directories_count,
              prefix,
              connector,
              depth
        );
      } else {
          self.add_file(
            entry,
            prefix,
            connector
          );
      }
    }
  }
  
  fn add_directory(
    &mut self,
    directory: &DirEntry,
    pos: &usize,
    directories_count: &usize,
    prefix: &str,
    connector: &str,
    depth: u16
  ) {
    let os_separator = match std::env::consts::OS {
      "windows" => "\\",
      _ => "/"
    };
    let dir_name = directory.file_name();
    self.add_line(true, format!("{prefix}{connector} {}{os_separator}", dir_name.to_str().unwrap()));
    let this_prefix = if (directories_count - 1).ne(pos) {
      format!("{prefix}{PIPE_PREFIX}")
    } else {
      format!("{prefix}{SPACE_PREFIX}")
    };
    self.build_tree_body(directory.path(), &this_prefix, depth + 1);
    self.total_directories += 1;
  }
  
  fn add_file(
    &mut self,
    file: &DirEntry,
    prefix: &str,
    connector: &str
  ) {
    let file_name = file.file_name();
    self.add_line(false, format!("{prefix}{connector} {}", file_name.to_str().unwrap()));
    self.total_files += 1;
  }

  fn exclude_dir(&self, current_dir: &DirEntry) -> bool {
    if current_dir.file_type().is_ok() && current_dir.file_type().unwrap().is_dir() {
      !&self.exclude_dirs.contains(&current_dir.file_name().to_str().unwrap().to_owned())
    } else {
      true
    }
  }
  
  fn filter_extension(&self, current_file: &DirEntry) -> bool {
    if current_file.file_type().unwrap().is_dir() {
      return true;
    }
    let file_name = &current_file.file_name();
    let file_path = std::path::Path::new(file_name);
    let file_extension = file_path.extension();
    
    if file_extension.is_none() {
      return true;
    }
    
    return !&self.filter_extensions.contains(&file_extension.unwrap().to_str().unwrap().to_string());
  }
  
  fn add_line(&mut self, is_dir: bool, text: String) {
    if !self.colored {
      self.tree.push(text);
    } else {
      match is_dir {
        true => self.tree.push(text.red().bold().to_string()),
        false => self.tree.push(text.green().to_string())
      };
    }
  }
}