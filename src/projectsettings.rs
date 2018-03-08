use std::path::PathBuf;

struct MemorySection {
    name: String,
    size: u32,
    addr: u32,
}

enum ProjectType {
    Rust,
    C,
    CXX,
    Python,
}

enum Framework {
    CopyAll,
    CopyNecessary,
    MakefileReference,
    SymlinkReference,
}

enum Modification {
    Ask,
    Modify,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectSettings {
    stackSize: u32,
    stackAddr: u32,
    heapSize: u32,
    heapAddr: u32,

    memorySections: Vec<MemorySection>,
    projectName: String,
    projectPath: PathBuf,
    resourcesPath: PathBuf,

    modificationStartegy: Modification,
    generateSeparately: bool,
    backupFiles: bool,
    deleteOrphans: bool,
    keepUserCode: bool,
}
