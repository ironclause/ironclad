use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::error::{IroncladError, IroncladResult};
use crate::project::compile_unit::CompileUnit;

pub mod compile_unit;
pub mod compiler_opts;

#[derive(Default, Debug)]
pub struct ErlProjectImpl {
    /// Global input file masks and compile options provided from the project file and command line
    pub input_masks: VecDeque<PathBuf>,

    /// Input directories which will be scanned for *.erl, ... (i.e. all masks from input_masks)
    pub input_directories: VecDeque<PathBuf>,

    pub compiler_opts: toml::Table,

    /// Collection of loaded modules
    pub modules: RwLock<HashMap<String, CompileUnit>>,
    // /// Stores files recently loaded from disk
    // pub file_cache: FileCache,

    /// List of all scanned input files in the order they were found
    pub input_paths: Vec<PathBuf>,
}

impl ErlProjectImpl {
    pub fn new() -> Self {
        // TODO: Load the configuration file
        Self {
            input_masks: VecDeque::new(),
            input_directories: VecDeque::new(),
            compiler_opts: Default::default(),
            modules: RwLock::new(HashMap::new()),
            input_paths: Vec::new(),
        }
    }

    /// Default file dict capacity
    pub const DEFAULT_CAPACITY: usize = 1024; // preallocate this many inputs in the file_list

    /// Traverse directories starting from each of the inputs.directories;
    /// Add files from inputs if not duplicate.
    pub fn build_file_list(&mut self) -> IroncladResult<()> {
        let mut file_set: HashSet<PathBuf> = HashSet::with_capacity(ErlProjectImpl::DEFAULT_CAPACITY);
        let mut file_list = Vec::new();

        for file_mask in &self.input_masks {
            for dir in &self.input_directories {
                let file_glob = dir.join("/**/").join(file_mask);
                let g_result = glob::glob(file_glob.to_str().unwrap());

                for entry in g_result.map_err(IroncladError::from)? {
                    match entry {
                        Ok(path) => Self::maybe_add_path(&mut file_set, &mut file_list, path)?,
                        Err(err) => return Err(IroncladError::from(err).into()),
                    }
                } // for glob search results
            } // for input dirs
        } // for input file masks

        self.input_paths = file_list;
        Ok(())
    }

    /// Check exclusions in the Self.input. Hashset is used to check for duplicates.
    /// Add to the file_list Vec to preserve order.
    fn maybe_add_path(
        file_set: &mut HashSet<PathBuf>,
        file_list: &mut Vec<PathBuf>,
        path: PathBuf,
    ) -> IroncladResult<()> {
        // Check duplicate
        let abs_path = std::fs::canonicalize(path).map_err(IroncladError::from)?;
        if file_set.contains(&abs_path) {
            return Ok(());
        }

        // Success: checks passed, add to the input list
        file_set.insert(abs_path.clone());
        file_list.push(abs_path);

        Ok(())
    }

    pub fn load_toml_config(&mut self, filename: &str) -> IroncladResult<()> {
        let config_contents = std::fs::read_to_string(filename).map_err(IroncladError::from)?;
        // Load TOML config as a generic map (TODO: Load as serde structured TOML)
        self.compiler_opts = toml::from_str::<toml::Table>(config_contents.as_str())?;
        Ok(())
    }
}

/// Wrapper for shared access
pub type ErlProject = Arc<ErlProjectImpl>;

impl Display for ErlProjectImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let paths = &self.compiler_opts.get("include_paths");
        write!(f, "ErlProject[glob_include={:?}]", paths)
    }
}
