use serde::Deserialize;

/// Options for building entire project, or a single module
/// This version of struct is parsed from TOML and all optional fields are Option<>
/// The real config is in the module above this.
#[derive(Default, Deserialize, Debug)]
pub struct IroncladProjectFile {
    pub compiler_options: CompilerOptions,
}

#[derive(Default, Deserialize, Debug)]
pub struct CompilerOptions {
    /// Directories to scan for input files. Default empty.
    pub input_paths: Option<Vec<String>>,
    /// File masks to scan for input files, do not add include files here only modules. Default *.erl.
    pub input_masks: Option<Vec<String>>,
    /// Defaults to empty list. Preprocessor defs in form of "NAME" or "NAME=VALUE"
    pub defines: Option<toml::Table>,
}

// impl CompilerOpts {
//     // pub(crate) fn load_toml_config(&self, config_contents: &str) -> IroncladResult<()> {
//     //     Ok(())
//     // }
//     pub fn from() -> Self {
//         Self { values: HashMap::new() }
//     }
// }
//
// impl CompilerOpts {
//     pub fn new() -> Self {
//         Self { values: HashMap::new() }
//     }
//
//     pub fn get(&self, key: &str) -> Option<&OptionValue> {
//         self.values.get(key)
//     }
//
//     pub fn get_clone(&self, key: &str) -> Option<OptionValue> {
//         self.values.get(key).cloned()
//     }
// }
