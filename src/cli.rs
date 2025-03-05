use crate::domain::LicenseArgs;

use clap::{Arg, ArgAction, Command, value_parser};
use std::path::PathBuf;

pub fn parse_args() -> LicenseArgs {
    let matches = Command::new("cd-allow")
	.arg(
	    Arg::new("filepath")
		.long("file")
		.default_value("deny.toml")
		.value_parser(value_parser!(PathBuf))
		.help("The path to the deny.toml file"),
	)
	.arg(
	    Arg::new("osi")
		.short('o')
		.long("osi")
		.action(ArgAction::SetTrue)
		.help("Allow OSI-approved licenses"),
	)
	.arg(
	    Arg::new("fsf")
		.short('f')
		.long("fsf")
		.action(ArgAction::SetTrue)
		.help("Allow FSF-approved licenses"),
	)
	.arg(
	    Arg::new("deprecated")
		.short('d')
		.long("deprecated")
		.action(ArgAction::SetTrue)
		.help("Allow Deprecated licenses"),
	)
	.get_matches();

    let filename: &PathBuf = matches.get_one("filepath").unwrap();
    let osi: bool = *matches.get_one("osi").unwrap();
    let fsf: bool = *matches.get_one("fsf").unwrap();
    let deprecated: bool = *matches.get_one("deprecated").unwrap();

    LicenseArgs {
	filepath: filename.clone(),
	osi,
	fsf,
	deprecated,
    }
}
