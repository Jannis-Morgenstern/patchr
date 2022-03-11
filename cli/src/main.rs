mod validators;

use clap::{Arg, Command};
use validators::is_valid_package_name;

fn main() {
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
                        .possible_values(["npm", "yarn", "pnpm", "auto"])
                        .default_value("auto")
                        .ignore_case(true)
                        .value_name("package-manager")
                        .multiple_values(false)
                        .help("The package manager of the current project.")
                        .long_help(
                            "The package manager of the current project.
Can either be specified explicitly or determined be a present lockfile.
This flag will always override the lock file.",
                        )
                        .display_order(1),
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
                        .display_order(2),
                )
                .arg(
                    Arg::new("exclude")
                        .short('e')
                        .long("exclude")
                        .takes_value(true)
                        .multiple_values(false),
                )
                .arg(
                    Arg::new("include")
                        .short('i')
                        .long("include")
                        .takes_value(true)
                        .multiple_values(false),
                )
                .arg(
                    Arg::new("create-issue")
                        .short('c')
                        .long("create-issue")
                        .takes_value(true)
                        .multiple_values(false),
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
            println!("{:?}", create_matches)
        }
        Some(("apply", apply_matches)) => {
            println!("{:?}", apply_matches)
        }
        _ => unreachable!(),
    }
}
