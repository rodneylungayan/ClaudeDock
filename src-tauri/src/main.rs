#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}, process::Command, sync::Mutex};
use tauri::State;
use uuid::Uuid;

#[cfg(windows)] use std::os::windows::{fs::MetadataExt, process::CommandExt};

#[derive(Clone, Serialize, Deserialize)]
struct Account { id: String, name: String, kind: String }
#[derive(Default, Serialize, Deserialize)]
struct Store { accounts: Vec<Account> }
struct AppState { lock: Mutex<()> }

fn normal_dir(path: &Path) -> Result<(), String> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.is_dir() || m.file_type().is_symlink() { return Err("Expected a normal directory".into()); }
    #[cfg(windows)] if m.file_attributes() & 0x400 != 0 { return Err("Reparse points are not allowed for profile directories".into()); }
    Ok(())
}
fn paths() -> Result<(PathBuf, PathBuf), String> {
    let root = dirs::data_local_dir().ok_or("Cannot find local application data")?.join("AccountProfileManager");
    fs::create_dir_all(&root).map_err(|e| e.to_string())?; normal_dir(&root)?;
    let profiles = root.join("profiles"); fs::create_dir_all(&profiles).map_err(|e| e.to_string())?; normal_dir(&profiles)?;
    Ok((root, profiles))
}
fn read_store(root: &Path) -> Result<Store, String> {
    let path = root.join("accounts.json");
    if !path.exists() { return Ok(Store::default()); }
    let m = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if !m.is_file() || m.file_type().is_symlink() { return Err("Unsafe account metadata file".into()); }
    #[cfg(windows)] if m.file_attributes() & 0x400 != 0 { return Err("Unsafe account metadata file".into()); }
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| format!("Cannot read account metadata: {e}"))
}
fn save_store(root: &Path, store: &Store) -> Result<(), String> {
    let path = root.join("accounts.json");
    // Never silently overwrite an unreadable store. Backup is retained before writes.
    if path.exists() { fs::copy(&path, root.join("accounts.backup.json")).map_err(|e| e.to_string())?; }
    fs::write(path, serde_json::to_vec_pretty(store).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn valid_input(name: &str, kind: &str) -> Result<(), String> {
    if name.trim().is_empty() || name.chars().count() > 80 || name.chars().any(char::is_control) { return Err("Name must be 1–80 visible characters".into()); }
    if !["Personal", "Work", "Custom"].contains(&kind) { return Err("Invalid account type".into()); } Ok(())
}
fn valid_id(id: &str) -> Result<(), String> {
    if id.len() != 32 || !id.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) { return Err("Invalid profile ID".into()); } Ok(())
}
fn find_claude() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Cannot find user directory")?;
    let appdata = dirs::config_dir().ok_or("Cannot find application data")?;
    [home.join(".local/bin/claude.exe"), appdata.join("npm/node_modules/@anthropic-ai/claude-code/bin/claude.exe")].into_iter().find(|p| p.is_file()).ok_or_else(|| "Claude Code was not found in the standard per-user native or npm installation locations.".into())
}
#[tauri::command] fn list_accounts(state: State<AppState>) -> Result<Vec<Account>, String> { let _g = state.lock.lock().map_err(|e| e.to_string())?; let (root, _) = paths()?; Ok(read_store(&root)?.accounts) }
#[tauri::command] fn add_account(state: State<AppState>, name: String, kind: String) -> Result<(), String> {
    valid_input(&name, &kind)?; let _g = state.lock.lock().map_err(|e| e.to_string())?; let (root, profiles) = paths()?; let mut store = read_store(&root)?;
    let id = Uuid::new_v4().simple().to_string(); fs::create_dir(profiles.join(&id)).map_err(|e| e.to_string())?;
    store.accounts.push(Account { id, name: name.trim().into(), kind }); save_store(&root, &store)
}
#[tauri::command] fn edit_account(state: State<AppState>, id: String, name: String, kind: String) -> Result<(), String> {
    valid_id(&id)?; valid_input(&name, &kind)?; let _g = state.lock.lock().map_err(|e| e.to_string())?; let (root, _) = paths()?; let mut store = read_store(&root)?;
    let a = store.accounts.iter_mut().find(|a| a.id == id).ok_or("Profile not found")?; a.name = name.trim().into(); a.kind = kind; save_store(&root, &store)
}
#[tauri::command] fn remove_account(state: State<AppState>, id: String) -> Result<(), String> {
    valid_id(&id)?; let _g = state.lock.lock().map_err(|e| e.to_string())?; let (root, _) = paths()?; let mut store = read_store(&root)?;
    let old = store.accounts.len(); store.accounts.retain(|a| a.id != id); if old == store.accounts.len() { return Err("Profile not found".into()); } save_store(&root, &store)
}
#[tauri::command] fn diagnostics() -> String { match find_claude() { Ok(p) => format!("Claude Code found: {}", p.display()), Err(e) => e } }
#[tauri::command] fn launch_code(state: State<AppState>, id: String, working_directory: String) -> Result<(), String> {
    valid_id(&id)?; let _g = state.lock.lock().map_err(|e| e.to_string())?; let (root, profiles) = paths()?;
    if !read_store(&root)?.accounts.iter().any(|a| a.id == id) { return Err("Profile not found".into()); }
    let config = profiles.join(id); normal_dir(&config)?; let working = PathBuf::from(working_directory); if !working.is_absolute() || !working.is_dir() { return Err("Enter an existing absolute working directory".into()); }
    let mut cmd = Command::new(find_claude()?); cmd.current_dir(working).env("CLAUDE_CONFIG_DIR", config);
    for key in ["ANTHROPIC_API_KEY", "ANTHROPIC_AUTH_TOKEN", "CLAUDE_CODE_OAUTH_TOKEN", "CLAUDE_CODE_USE_BEDROCK", "CLAUDE_CODE_USE_VERTEX", "CLAUDE_CODE_USE_FOUNDRY"] { cmd.env_remove(key); }
    #[cfg(windows)] cmd.creation_flags(0x00000010); // CREATE_NEW_CONSOLE
    cmd.spawn().map_err(|e| format!("Could not start Claude Code: {e}"))?; Ok(())
}
fn main() { tauri::Builder::default().manage(AppState { lock: Mutex::new(()) }).invoke_handler(tauri::generate_handler![list_accounts, add_account, edit_account, remove_account, diagnostics, launch_code]).run(tauri::generate_context!()).expect("failed to run application"); }
