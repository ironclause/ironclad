use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::{RwLock};
use erl_parse::{TokenReader};
use erl_pp::Preprocessor;
use erl_tokenize::Lexer;
use trackable::track_try_unwrap;
use crate::error::{IroncladError, IroncladResult};
use crate::project::compile_unit::CompileUnit;
use crate::project::compiler_opts::IroncladProjectFile;

pub mod compile_unit;
pub mod compiler_opts;

#[derive(Default, Debug)]
pub struct ErlProjectImpl {
    // / Global input file masks and compile options provided from the project file and command line
    // pub input_masks: VecDeque<PathBuf>,

    // / Input directories which will be scanned for *.erl, ... (i.e. all masks from input_masks)
    // pub input_directories: VecDeque<PathBuf>,

    // As loaded from ironclad.toml
    pub project_conf: IroncladProjectFile,

    /// Collection of loaded modules
    pub modules: RwLock<HashMap<String, CompileUnit>>,
    // /// Stores files recently loaded from disk
    // pub file_cache: FileCache,

    /// List of all scanned input files in the order they were found
    pub input_files: Vec<PathBuf>,
    /// Copied from project_conf.exclude_prefixes but with a default value
    exclude_prefixes: Vec<String>,
    /// Copied from project_conf.exclude_suffixes but with a default value
    exclude_suffixes: Vec<String>,
}

impl ErlProjectImpl {
    pub fn new() -> Self {
        // TODO: Load the configuration file
        Self {
            //input_masks: VecDeque::new(),
            // input_directories: VecDeque::new(),
            project_conf: Default::default(),
            modules: RwLock::new(HashMap::new()),
            input_files: Vec::new(),
            exclude_prefixes: Vec::default(),
            exclude_suffixes: Vec::default(),
        }
    }

    /// Default file dict capacity
    pub const DEFAULT_CAPACITY: usize = 1024; // preallocate this many inputs in the file_list

    /// Traverse directories starting from each of the inputs.directories; Add files from inputs if not duplicate.
    /// Assign the result of this function to 'self.input_paths'.
    pub fn build_file_list(&self) -> IroncladResult<Vec<PathBuf>> {
        let mut file_set: HashSet<PathBuf> = HashSet::with_capacity(ErlProjectImpl::DEFAULT_CAPACITY);
        let mut file_list = Vec::new();
        let input_paths = self.project_conf.compiler_options.input_paths.clone().unwrap_or_default();

        let m_input_masks = self.project_conf.compiler_options.input_masks.as_ref();
        // println!("Building list of input files... input_paths={:?} input_masks={:?}", input_paths, m_input_masks);

        if let Some(input_masks) = m_input_masks {
            for file_mask in input_masks {
                for dir in input_paths.iter() {
                    let file_glob = PathBuf::from(dir).join("**").join(file_mask);
                    // println!("Dir {:?} Glob: {:?}", dir, file_glob);

                    let g_result = glob::glob(file_glob.to_str().unwrap());

                    for entry in g_result.map_err(IroncladError::from)? {
                        match entry {
                            Ok(path) => self.maybe_add_path(&mut file_set, &mut file_list, path)?,
                            Err(err) => return Err(IroncladError::from(err).into()),
                        }
                    } // for glob search results
                } // for input dirs
            } // for input file masks
        }
        Ok(file_list)
    }

    fn is_excluded(&self, path: &Path) -> bool {
        for exclude in &self.exclude_prefixes {
            if path.starts_with(exclude) {
                return true;
            }
        }
        for exclude in &self.exclude_suffixes {
            if path.ends_with(exclude) {
                return true;
            }
        }
        false
    }

    /// Check exclusions in the Self.input. Hashset is used to check for duplicates.
    /// Add to the file_list Vec to preserve order.
    fn maybe_add_path(
        &self,
        file_set: &mut HashSet<PathBuf>,
        file_list: &mut Vec<PathBuf>,
        path: PathBuf,
    ) -> IroncladResult<()> {
        // Check duplicate
        let abs_path = std::fs::canonicalize(path).map_err(IroncladError::from)?;
        if file_set.contains(&abs_path) {
            return Ok(());
        }
        if self.is_excluded(&abs_path) {
            return Ok(());
        }

        // Success: checks passed, add to the input list
        file_set.insert(abs_path.clone());
        file_list.push(abs_path);

        Ok(())
    }

    pub fn load_project_config(&mut self, filename: &str) -> IroncladResult<()> {
        let config_contents = std::fs::read_to_string(filename).map_err(IroncladError::from)?;
        // Load TOML config as a generic map (TODO: Load as serde structured TOML)
        self.project_conf = toml::from_str(config_contents.as_str())?;
        self.exclude_prefixes = self.project_conf.compiler_options.exclude_prefixes
            .as_ref().unwrap_or(&Vec::new()).clone();
        self.exclude_suffixes = self.project_conf.compiler_options.exclude_suffixes
            .as_ref().unwrap_or(&Vec::new()).clone();
        Ok(())
    }

    /// Convert input files constructed in build_file_list() into source trees stored in `CompileUnit`s
    pub(crate) fn parse_inputs(&self) -> IroncladResult<()> {
        // println!("Parsing input files... {:?}", self.input_files);
        for path in self.input_files.iter() {
            let file_contents = std::fs::read_to_string(path.as_path()).map_err(IroncladError::from)?;
            self.parse_module_text(path, file_contents.as_str());
        }
        Ok(())
    }

    fn parse_module_text(&self, filename: &Path, text: &str) {
        println!("* Parsing {}", filename.to_string_lossy());

        // let mut parser = Parser::new(TokenReader::new(Preprocessor::new(Lexer::new(text))));
        let mut pp = Preprocessor::new(Lexer::new(text));
        // Add current file directory to include search path
        if let Some(parent) = filename.parent() {
            println!("    adding include dir {:?}", parent);
            pp.code_paths_mut().push_back(parent.into()); // add include dirs
        }
        let reader = &mut TokenReader::new(&mut pp);
        let _module = track_try_unwrap!(erl_parse::builtin::parse_module(reader));

        // let value: Form = track_try_unwrap!(parser.parse(), "text={:?}", text);
    }
}

// / Wrapper for shared access
// pub type ErlProject = Arc<ErlProjectImpl>;

impl Display for ErlProjectImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let paths = self.project_conf.compiler_options.input_paths.as_ref();
        write!(f, "ErlProject[glob_include={:?}, inp_paths={:?}]", paths, self.input_files)
    }
}
