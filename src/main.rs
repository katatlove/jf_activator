use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Config {
    aircraft_details: AircraftDetails,
}

#[derive(Debug, Deserialize)]
struct AircraftDetails {
    full_name: String,
    generic_name: String,
}

fn splash(generic_name: &str) {
    println!("      ██╗██╗   ██╗███████╗████████╗███████╗██╗     ██╗ ██████╗ ██╗  ██╗████████╗");
    println!("      ██║██║   ██║██╔════╝╚══██╔══╝██╔════╝██║     ██║██╔════╝ ██║  ██║╚══██╔══╝");
    println!("      ██║██║   ██║███████╗   ██║   █████╗  ██║     ██║██║  ███╗███████║   ██║");
    println!(" ██   ██║██║   ██║╚════██║   ██║   ██╔══╝  ██║     ██║██║   ██║██╔══██║   ██║");
    println!(" ╚█████╔╝╚██████╔╝███████║   ██║   ██║     ███████╗██║╚██████╔╝██║  ██║   ██║");
    println!("  ╚════╝  ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝");
    println!();
    println!("Activator based on SimPlaza");
    println!("For the {}", generic_name);
    println!();
}

fn parent_folder() -> io::Result<String> {
    let exe_path: PathBuf = env::current_exe()?;

    let parent: &std::path::Path = exe_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Executable has no parent folder"))?;

    let folder: &std::ffi::OsStr = parent
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not determine folder name"))?;

    Ok(folder.to_string_lossy().into_owned())
}

fn pause() -> io::Result<()> {
    print!("Press Enter to continue . . . ");
    io::stdout().flush()?;

    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Simulators {
    Steam,
    MicrosoftStore,
    None,
}

fn steam_path() -> io::Result<PathBuf> {
    let appdata: String = env::var("APPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "APPDATA not found"))?;

    Ok(PathBuf::from(appdata).join("Microsoft Flight Simulator"))
}

fn microsoft_store_path() -> io::Result<PathBuf> {
    let local_appdata: String = env::var("LOCALAPPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA not found"))?;

    Ok(PathBuf::from(local_appdata)
        .join("Packages")
        .join("Microsoft.Limitless_8wekyb3d8bbwe"))
}

fn fs_type() -> io::Result<Simulators> {
    if steam_path()?.exists() {
        Ok(Simulators::Steam)
    } else if microsoft_store_path()?.exists() {
        Ok(Simulators::MicrosoftStore)
    } else {
        Ok(Simulators::None)
    }
}

fn make_steam(full_name: &str) -> io::Result<()> {
    let path: PathBuf = steam_path()?.join("Packages").join(full_name).join("work");

    fs::create_dir_all(path)
}

fn userinfo_path(full_name: &str) -> io::Result<PathBuf> {
    let mut path: PathBuf = env::current_exe()?;

    path.pop();
    path.push("ContentInfo");
    path.push(full_name);
    path.push("userinfo.txt");

    Ok(path)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        let _ = pause();
    }
}

fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = env::current_exe()?;
    config_path.pop();
    config_path.push("jf_activator.toml");

    let config_text = fs::read_to_string(config_path)?;

    let config: Config =
        toml::from_str(&config_text).map_err(|e| format!("Invalid TOML configuration: {e}"))?;

    splash(&config.aircraft_details.generic_name);

    let aircraft: &AircraftDetails = &config.aircraft_details;

    if parent_folder()? != aircraft.full_name {
        println!(
            "Please rename the root aircraft folder to {}",
            aircraft.full_name
        );

        pause()?;
        return Ok(());
    }

    let userinfo: PathBuf = userinfo_path(&aircraft.full_name)?;

    if !userinfo.exists() {
        println!(
            "userinfo.txt could not be found. Your files are either corrupted or incomplete, please redownload and/or reextract the original archive."
        );

        pause()?;
        return Ok(());
    }

    match fs_type()? {
        Simulators::Steam => match make_steam(&aircraft.full_name) {
            Ok(()) => {
                let path: PathBuf = steam_path()?
                    .join("Packages")
                    .join(&aircraft.full_name)
                    .join("work")
                    .join("userinfo.txt");

                fs::copy(
                    userinfo_path(&aircraft.full_name)
                        .expect("Could not access userinfo.txt path."),
                    &path,
                )?;

                println!(
                    "Successfully installed the {} for MSFS Steam Version.",
                    aircraft.generic_name
                );
            }
            Err(e) => {
                println!(
                    "An error occurred while trying to install the {} for MSFS Steam Version: {}",
                    aircraft.generic_name, e
                );
                pause()?;
                println!("8");
                return Err(Box::new(e));
            }
        },

        Simulators::MicrosoftStore => {
            println!("Microsoft Store installation is not implemented yet.");
        }

        Simulators::None => {
            println!("No simulator was detected on your hard drive.");
            pause()?;
            return Ok(());
        }
    }

    pause()?;
    Ok(())
}
