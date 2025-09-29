use std::io;
use std::path;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn open_on_explorer(folder_path: &path::Path) -> io::Result<std::process::Child> {
    Command::new("xdg-open").arg(folder_path).spawn()
}

#[cfg(target_os = "windows")]
pub fn open_on_explorer(folder_path: &path::Path) -> io::Result<std::process::Child> {
    std::process::Command::new("explorer")
        .arg(folder_path)
        .spawn()
}

#[cfg(target_os = "linux")]
pub fn get_steam_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Could not get HOME dir to get steam path.")?;
    return Ok(home_dir.join(".local/share/Steam"));
}

#[cfg(target_os = "windows")]
pub fn get_steam_path() -> Result<PathBuf, String> {
    // echo %programfiles(x86)
    let programfilepath = std::env::var("programfilesx86").map_err(|x| x.to_string())?;
    let programfiles = path::Path::new(&programfilepath);

    Ok(programfiles.join("Steam"))
}

pub fn get_steam_compatdata() -> Result<PathBuf, String> {
    let steam_path = get_steam_path()?;
    Ok(steam_path.join("steamapps").join("compatdata"))
}

pub fn get_steam_common() -> Result<PathBuf, String> {
    let steam_path = get_steam_path()?;
    Ok(steam_path.join("steamapps").join("common"))
}
