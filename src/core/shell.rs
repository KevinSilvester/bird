use get_shell;
use crate::utils::errors::BirdError;

#[cfg(unix)]
const DEFAULT_BACKUP_SHELL: &str = "bash";

#[cfg(windows)]
const DEFAULT_BACKUP_SHELL: &str = "powershell";

pub enum Shell {
   
   // pub const AVAILABLE_SHELLS: &[&str; 9] = &["cmd", "bash", "zsh", "fish", "powershell", "pwsh", "xonsh", "nu", "elvish"];
   // pub const AVAILABLE_SHELLS: &[&str; 9] = &["bash", "zsh", "fish", "powershell", "pwsh", "xonsh", "nu", "elvish", "ion"];
   Cmd,
   Bash,
   Zsh,
   Fish,
   Powershell,
   Pwsh,
   Xonsh,
   Nushell,
   Elvish
}

impl Shell {
   pub fn default() -> String {
      match get_shell::get_shell_name() {
         Ok(sh) => sh,
         Err(_) => DEFAULT_BACKUP_SHELL.to_owned()
      }
   }

   pub fn possible_values() -> &'static [&'static str] {
      #[cfg(windows)]
      return &["cmd", "bash", "zsh", "fish", "powershell", "pwsh", "xonsh", "nu", "elvish"];

      #[cfg(unix)]
      return &["ion", "bash", "zsh", "fish", "powershell", "pwsh", "xonsh", "nu", "elvish"];
   }
}
