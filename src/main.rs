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
    Msfs2024Steam,
    Msfs2024Store,
    Msfs2020Steam,
    Msfs2020Store,
    None,
}

fn steam20_path() -> io::Result<PathBuf> {
    let appdata: String = env::var("APPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "APPDATA not found"))?;

    Ok(PathBuf::from(appdata).join("Microsoft Flight Simulator"))
}

fn microsoft_store20_path() -> io::Result<PathBuf> {
    let local_appdata: String = env::var("LOCALAPPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA not found"))?;

    Ok(PathBuf::from(local_appdata)
        .join("Packages")
        .join("Microsoft.Limitless_8wekyb3d8bbwe"))
}

fn steam24_path() -> io::Result<PathBuf> {
    let appdata = env::var("APPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "APPDATA not found"))?;

    Ok(PathBuf::from(appdata).join("Microsoft Flight Simulator 2024"))
}

fn microsoft_store24_path() -> io::Result<PathBuf> {
    let local_appdata = env::var("LOCALAPPDATA")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA not found"))?;

    Ok(PathBuf::from(local_appdata)
        .join("Packages")
        .join("Microsoft.Limitless_8wekyb3d8bbwe")
        .join("LocalState"))
}

fn fs_type() -> io::Result<Simulators> {
    if steam24_path()?.join("WASM").exists() {
        return Ok(Simulators::Msfs2024Steam);
    }

    if microsoft_store24_path()?.join("WASM").exists() {
        return Ok(Simulators::Msfs2024Store);
    }

    if steam20_path()?.join("Packages").exists() {
        return Ok(Simulators::Msfs2020Steam);
    }

    if microsoft_store20_path()?
        .join("LocalState")
        .join("packages")
        .exists()
    {
        return Ok(Simulators::Msfs2020Store);
    }

    Ok(Simulators::None)
}

fn make_install_path(sim: &Simulators, full_name: &str) -> io::Result<PathBuf> {
    let path = match sim {
        Simulators::Msfs2024Steam => steam24_path()?
            .join("WASM")
            .join("MSFS2024")
            .join(full_name)
            .join("work"),

        Simulators::Msfs2024Store => microsoft_store24_path()?
            .join("WASM")
            .join("MSFS2024")
            .join(full_name)
            .join("work"),

        Simulators::Msfs2020Steam => steam20_path()?
            .join("Packages")
            .join(full_name)
            .join("work"),

        Simulators::Msfs2020Store => microsoft_store20_path()?
            .join("LocalState")
            .join("packages")
            .join(full_name)
            .join("work"),

        Simulators::None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No simulator found",
            ));
        }
    };

    fs::create_dir_all(&path)?;

    Ok(path)
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

    let simulator = fs_type()?;

    match simulator {
        Simulators::Msfs2024Steam
        | Simulators::Msfs2024Store
        | Simulators::Msfs2020Steam
        | Simulators::Msfs2020Store => {
            let work_path = make_install_path(&simulator, &aircraft.full_name)?;

            let destination = work_path.join("userinfo.txt");

            fs::copy(userinfo_path(&aircraft.full_name)?, destination)?;

            println!("Successfully installed the {}.", aircraft.generic_name);
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
