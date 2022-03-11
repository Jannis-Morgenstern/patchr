mod path_util;
mod validators;

use clap::{Arg, ArgEnum, Command, PossibleValue};
use path_util::find_file_in_parent_dirs;
use validators::is_valid_package_name;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Auto,
}

impl PackageManager {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        PackageManager::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl std::str::FromStr for PackageManager {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

fn main() {
    let current_working_directory = std::env::current_dir().unwrap();
    let package_json_path = find_file_in_parent_dirs(&current_working_directory, "package.json");

    let matches = Command::new("patchr")
        .about("patches packages")
        .version("0.1.0")
        .author("Jannis Morgenstern")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("create patch file")
                .arg(
                    Arg::new("package")
                        .index(1)
                        .takes_value(true)
                        .value_name("package")
                        .multiple_values(false)
                        .required(true)
                        .validator(is_valid_package_name)
                        .help("The package that should be patched.")
                        .long_help(
                            "The package that should be patched.
Also works with organizations (`@org/package`).",
                        ),
                )
                .arg(
                    Arg::new("package-manager")
                        .short('p')
                        .long("package-manager")
                        .takes_value(true)
                        .possible_values(PackageManager::possible_values())
                        .default_value("auto")
                        .ignore_case(true)
                        .value_name("package-manager")
                        .multiple_values(false)
                        .help("The package manager of the current project.")
                        .long_help(
                            "The package manager of the current project.
Can either be specified explicitly or determined by a present lockfile.
This flag will always override the lock file.",
                        )
                        .display_order(1),
                )
                .arg(
                    Arg::new("project-dir")
                        .short('r')
                        .long("project-dir")
                        .takes_value(true)
                        .multiple_values(false)
                        .default_value("./")
                        .help("The root directory of the current project.")
                        .long_help(
                            "The root directory of the current project.\
Can either be specified explicitly or determined by the next package.json file in parent directories.
This flag will always override the location of the package.json file.",
                        )
                        .display_order(2),
                )
                .arg(
                    Arg::new("patch-dir")
                        .short('d')
                        .long("patch-dir")
                        .takes_value(true)
                        .multiple_values(false)
                        .default_value("./patches")
                        .help("The directory the patch file will be placed in.")
                        .long_help(
                            "The directory the patch file will be placed in.
Note: you can not place patched outside your project root directory.",
                        )
                        .display_order(3),
                )
                .arg(
                    Arg::new("exclude")
                        .short('e')
                        .long("exclude")
                        .takes_value(true)
                        .multiple_values(false)
                        .display_order(4),
                )
                .arg(
                    Arg::new("include")
                        .short('i')
                        .long("include")
                        .takes_value(true)
                        .multiple_values(false)
                        .display_order(5),
                )
                .arg(
                    Arg::new("create-issue")
                        .short('c')
                        .long("create-issue")
                        .takes_value(true)
                        .multiple_values(false)
                        .display_order(6),
                )
                .arg(
                    Arg::new("temporary-git-dir")
                        .short('t')
                        .long("temporary-git-dir")
                        .takes_value(true)
                        .multiple_values(false)
                        .display_order(7),
                ),
        )
        .subcommand(
            Command::new("apply").about("applies patch files").arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .takes_value(true)
                    .multiple_values(false),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", create_matches)) => {
            let package = create_matches.value_of("package").unwrap();
            let package_manager =
                PackageManager::from_str(create_matches.value_of("package-manager").unwrap(), true)
                    .unwrap();
            println!(
                "Package Manager: {:?}, Package: {}",
                package_manager, package
            );
            // println!("{:?}", package_json_path);
        }
        Some(("apply", apply_matches)) => {
            println!("{:?}", apply_matches)
        }
        _ => unreachable!(),
    }
}
