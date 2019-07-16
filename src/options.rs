use std::path::PathBuf;
use structopt::StructOpt;
use std::ffi::OsString;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "linecount")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Count files, folders and symbolic links in a folder.
pub struct Options {
    /// The starting directory. If empty, searches current directory.
    #[structopt(name = "directory", default_value = ".", parse(from_os_str))]
    pub dir: PathBuf,

    /// File extensions to count lines for.
    #[structopt(name = "extensions", short = "e", parse(from_os_str))]
    pub file_extensions: Vec<OsString>,

    /// Enter folders (traverse directory recursively)
    #[structopt(short = "r")]
    pub recursive: bool,
}