use std::path::{Path, PathBuf};
use std::env;

use cargo::Cargo;

#[derive(Serialize, Deserialize, Debug)]
enum CodeRegeneration {
    OverwriteAll,
    AskOnConflict,
    KeepUserCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSettings {
    project_name: String,
    project_path: PathBuf,
    resources_path: PathBuf,

    // File generation
    separate_inits: bool,
    backup_previous: bool,
    remove_orphans: bool,

    // Code generation
    keep_code: CodeRegeneration,
    diff3_path: PathBuf,

    // Memory options
    stack_size: u32,
    stack_addr: u32,
    heap_size: u32,
    heap_addr: u32,

    cargo: Cargo,
}

impl ProjectSettings {
    pub fn new() -> ProjectSettings {
        // TODO: pass as args
        let cwd = env::current_dir().unwrap();
        let res = env::current_dir().unwrap();
        let diff3 = env::current_dir().unwrap();

        ProjectSettings {
            project_name: String::new(),
            project_path: cwd,
            resources_path: res,
            separate_inits: false,
            backup_previous: true,
            remove_orphans: false,
            keep_code: CodeRegeneration::AskOnConflict,
            diff3_path: diff3,
            // TODO: Get MCUConf and get RAM type memory
            stack_addr: 0,
            stack_size: 0,
            heap_addr: 0,
            heap_size: 0,
            cargo: Cargo::new(),
        }
    }

    pub fn get_project_path(&self) -> &Path {
        &self.project_path
    }

    pub fn get_resources_path(&self) -> &Path {
        &self.resources_path
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let project_settings = ProjectSettings::new();
    }
}
