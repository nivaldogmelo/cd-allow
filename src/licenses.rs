use std::fs;

use anyhow::{Context, Error};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use toml_edit::{DocumentMut, Item};

use crate::domain::LicenseArgs;

#[derive(Deserialize, Serialize)]
struct DenyToml {
    licenses: Licenses,
}

#[derive(Deserialize, Serialize)]
struct Licenses {
    allow: Vec<String>,
}

pub async fn get_licenses(args: &LicenseArgs) -> Result<(), Error> {
    let filename = &args.filepath;
    let osi = &args.osi;
    let fsf = &args.fsf;
    let deprecated = &args.deprecated;

    let url = "https://raw.githubusercontent.com/spdx/license-list-data/v3.11/json/licenses.json";

    let response = reqwest::get(url)
	.await
	.context("Failed to fetch SPDX license list")?
	.json::<Value>()
	.await
	.context("Failed to parse JSON response")?;

    let mut licenses = Vec::new();

    if let Some(license_list) = response["licenses"].as_array() {
	for license in license_list {
	    if let (Some(id), Some(is_osi), Some(is_fsf_libre), Some(is_deprecated)) = (
		license["licenseId"].as_str(),
		license["isOsiApproved"].as_bool(),
		license["isFsfLibre"].as_bool(),
		license["isDeprecatedLicenseId"].as_bool(),
	    ) {
		if (*osi == is_osi) && (*fsf == is_fsf_libre) && (*deprecated == is_deprecated) {
		    if !(id.contains("-only") || id.contains("-or-later")) {
			licenses.push(id.to_string());
		    }
		}
	    }
	}
    }

    let toml_file = fs::read_to_string(filename).context("Failed to read deny.toml")?;

    let mut deny_toml = toml_file
	.parse::<DocumentMut>()
	.context("Failed to parse TOML")?;

    let licenses_toml = deny_toml
	.entry("licenses")
	.or_insert(Item::Table(Default::default()));

    let default_allow = Item::Value(toml_edit::Value::Array(Default::default()));

    let allow_toml = licenses_toml
	.as_table_mut()
	.context("Failed to access `[licenses]` section")?
	.entry("allow")
	.or_insert(default_allow);

    let allow_array = allow_toml
	.as_array_mut()
	.context("Failed to access `allow` array")?;

    for license in licenses.iter() {
	allow_array.push(license);
    }

    fs::write(filename, deny_toml.to_string()).context("Failed to write deny.toml")?;

    Ok(())
}
