// use std::collections::HashMap;
// use crate::error::IroncladResult;
//
// #[derive(Debug, Clone)]
// pub enum OptionValue {
//     None,
//     Bool(bool),
//     Int(i64),
//     // Float(f64),
//     String(String),
//     List(Vec<OptionValue>),
// }
//
// impl OptionValue {
//     pub fn as_list(&self) -> Option<Vec<OptionValue>> {
//         match self {
//             OptionValue::String(s) => Some(vec![OptionValue::String(s.clone())]),
//             OptionValue::List(list) => Some(list.clone()),
//             _ => None,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Default)]
// pub struct CompilerOpts {
//     values: HashMap<String, OptionValue>,
// }
//
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
