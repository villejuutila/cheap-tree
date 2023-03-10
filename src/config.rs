use clap::Parser;

#[derive(Parser, Debug)]
#[command(
  version = env!("CARGO_PKG_VERSION"),
  about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Args {
  /// Directory to build tree from.
  pub dir: Option<String>,
  
  /// Maximum depth of tree, defaults to 0.
  pub max_depth: Option<u16>,
  
  /// Flag to include only directories, defaults to false.
  #[clap(short, long)]
  pub dir_only: bool,
  
  /// Flag to include files/directories starting with ., defaults to false.
  #[clap(short, long)]
  pub include_hidden: bool,
  
  /// Flag to colorize tree. Directories will be red and files green. Defaults to false.
  #[clap(short, long)]
  pub colored: bool,
  
  /// Comma separated list of files/directories to exclude from tree. E.g. -e src,config
  #[clap(short, long, value_delimiter = ',')]
  pub exclude: Option<Vec<String>>,
  
  /// Comma separated list of file extensions to exclude from tree. E.g. -f png,js
  #[clap(short, long, value_delimiter = ',')]
  pub filter_extension: Option<Vec<String>>,
  
  /// Name of file to output tree into.
  #[clap(short, long)]
  pub output: Option<String>,
}