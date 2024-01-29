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

    cmd.arg("./samples/toto/");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(get_error_missing_folder_localize()));

    Ok(())
}

#[test]
fn file_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("./samples/small/");
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
        .stdout(predicate::str::contains("framels 0.7.0"));

    Ok(())
}

#[test]
fn cli_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-h");
    cmd.assert().success().stdout(predicate::str::contains(
        "a simple command line tool to list frame sequence in friendly way",
    ));

    Ok(())
}

#[test]
fn cli_listing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-l").arg("./samples/small/");
    cmd.assert().success().stdout(predicate::str::contains(
        "./samples/small/foo_bar.exr layer #0 w x h: 8 x 8;",
    ));

    Ok(())
}

#[test]
fn cli_listing_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-l").arg("-r").arg("./samples/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("layer #0 w x h: 8 x 8;"));

    Ok(())
}

#[test]
fn cli_tree() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-t").arg("-r").arg("./samples");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("┗ foo_bar.exr"));

    Ok(())
}

#[test]
fn cli_tree_with_listing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-t").arg("-l").arg("-r").arg("./samples");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("┗ foo_bar.exr"));

    Ok(())
}

#[test]
fn cli_tree_with_listing_and_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fls")?;

    cmd.arg("-t").arg("-l").arg("-r").arg("./samples");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("┗ foo_bar.exr"));

    Ok(())
}
