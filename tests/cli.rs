use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use sys_locale::get_locale;

fn get_error_missing_folder_localize() -> String {
    let locale: String = get_locale().unwrap_or_else(|| String::from("en-US"));
    if locale == "fr-FR" {
        "Le chemin d’accès spécifié est introuvable.".to_string()
    } else {
        "No such file or directory".to_string()
    }
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd: Command = Command::cargo_bin("fls")?;

    cmd.arg("--").arg("./samples/toto/");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(get_error_missing_folder_localize()));

    Ok(())
}

#[test]
fn file_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("--").arg("./samples/small/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("aaa.***.tif@1-5\nfoo_bar.exr"));

    Ok(())
}
#[test]
fn cli_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-V");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("framels 0.4.0-rc.7"));

    Ok(())
}
