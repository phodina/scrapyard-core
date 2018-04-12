use std::path::{Path, PathBuf};

use mcu::MCUConf;

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
    cargo: Cargo,
}

impl ProjectSettings {
    pub fn new(
        mcu_conf: &MCUConf,
        project_path: &Path,
        resources_path: &Path,
        diff3_path: &Path,
    ) -> ProjectSettings {
        ProjectSettings {
            project_name: String::new(),
            project_path: project_path.to_owned(),
            resources_path: resources_path.to_owned(),
            separate_inits: false,
            backup_previous: true,
            remove_orphans: false,
            keep_code: CodeRegeneration::AskOnConflict,
            diff3_path: diff3_path.to_owned(),
            cargo: Cargo::new(mcu_conf),
        }
    }

    pub fn set_project_path(&mut self, project_path: &Path) {
        self.project_path = project_path.to_owned();
    }

    pub fn get_project_path(&self) -> &Path {
        &self.project_path
    }

    pub fn set_resources_path(&mut self, resources_path: &Path) {
        self.resources_path = resources_path.to_owned();
    }

    pub fn get_resources_path(&self) -> &Path {
        &self.resources_path
    }

    pub fn get_cargo(&self) -> &Cargo {
        &self.cargo
    }

    pub fn set_separate_inits(&mut self, value: bool) {
        self.separate_inits = value;
    }

    pub fn get_separate_inits(&self) -> bool {
        self.separate_inits
    }

    pub fn set_backup_previous(&mut self, value: bool) {
        self.backup_previous = value;
    }

    pub fn get_backup_previous(&self) -> bool {
        self.backup_previous
    }

    pub fn set_remove_orphans(&mut self, value: bool) {
        self.remove_orphans = value;
    }

    pub fn get_remove_orphans(&self) -> bool {
        self.remove_orphans
    }

    pub fn set_diff3_path(&mut self, path: &Path) {
        self.diff3_path = path.to_owned();
    }

    pub fn get_diff3_path(&self) -> &Path {
        &self.diff3_path.as_path()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use mcu::MCU;

    #[test]
    fn it_works() {
        let sample = Path::new("./samples/STM32F030C6Tx.json");
        let mcu = MCU::new(sample).unwrap();

        let mcu_conf = mcu.finish();
        let project_path = Path::new("");
        let templates_path = Path::new("");
        let diff3_path = Path::new("");

        let project_settings =
            ProjectSettings::new(&mcu_conf, &project_path, &templates_path, &diff3_path);
    }
}
