use std::io;
use std::path;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

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
pub fn open_file(file_path: &path::Path) -> io::Result<std::process::Child> {
    Command::new("xdg-open").arg(file_path).spawn()
}

#[cfg(target_os = "windows")]
pub fn open_file(folder_path: &path::Path) -> io::Result<std::process::Child> {
    Command::new("cmd")
        .args(["/C", "start", folder_path.to_str().unwrap()])
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
    let programfilepath = std::env::var("programfiles(x86)").map_err(|x| x.to_string())?;
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

pub fn get_unix_timestamp_secs() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}

pub fn generate_display_name_from_key(save_key: &str) -> String {
    // Replace underscores and hyphens with spaces, then capitalize each word
    let mut display_name = save_key
        .replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Optional: ensure the result isn't empty (just in case)
    if display_name.is_empty() {
        display_name = save_key.to_string();
    }

    display_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_fallback_test() {
        assert_eq!(generate_display_name_from_key("wargroove"), "Wargroove");
        assert_eq!(
            generate_display_name_from_key("hollow_knight"),
            "Hollow Knight"
        );
        assert_eq!(
            generate_display_name_from_key("hollow-knight-silksong"),
            "Hollow Knight Silksong"
        );
    }
}
