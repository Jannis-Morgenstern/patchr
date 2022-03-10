use clap::{ArgEnum, Parser};

#[derive(Debug, Clone, ArgEnum)]
enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

#[derive(Parser, Debug)]
struct Args {
    /// The package to create the patch from
    package: String,

    /// The package manager your project uses
    #[clap(arg_enum, long)]
    package_manager: PackageManager,

    /// The directory the patch files will be placed in
    #[clap(parse(from_os_str), long)]
    patch_dir: std::path::PathBuf,

    /// exclude
    #[clap(long)]
    exclude: String,

    /// include
    #[clap(long)]
    include: String,

    /// create issue
    #[clap(long)]
    create_issue: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}
