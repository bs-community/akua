use git2::Deltas;
use std::process::Command;

pub struct Requirements {
    composer: bool,
    yarn: bool,
    build: bool,
}

impl Requirements {
    pub fn new() -> Requirements {
        Requirements {
            composer: false,
            yarn: false,
            build: false,
        }
    }

    pub fn from_deltas(deltas: Deltas) -> Requirements {
        deltas.fold(Requirements::new(), |mut req, diff| {
            let path = diff.new_file().path().expect("Failed to diff.");
            if path.starts_with("resources/assets/src") {
                req.build = true;
            } else if path.starts_with("yarn.lock") {
                req.yarn = true;
            } else if path.starts_with("composer.lock") {
                req.composer = true;
            }

            req
        })
    }
}

pub fn install(requirements: Requirements) -> std::io::Result<()> {
    if requirements.yarn {
        println!("Installing front end dependencies...");
        Command::new("yarn").spawn()?;
    }
    if requirements.build || requirements.yarn {
        println!("Building front end assets...");
        Command::new("yarn").arg("build").spawn()?;
    }
    if requirements.composer {
        println!("Installing PHP dependencies...");
        Command::new("composer")
            .args(vec!["install", "--no-dev"])
            .spawn()?;
    }

    Ok(())
}
