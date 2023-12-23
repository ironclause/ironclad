mod error;
mod project;

use std::process::exit;
use crate::error::IroncladResult;
use crate::project::{ErlProjectImpl};

fn main() {
    match main_do() {
        Ok(_) => {
            println!("Ironclad finished.");
            exit(0);
        }
        Err(e) => {
            println!("{}", e);
            exit(e.get_process_exit_code())
        }
    }
}

fn main_do() -> IroncladResult<()> {
    let mut project = ErlProjectImpl::new();
    project.load_project_config("ironclad.toml")?;
    project.input_paths = project.build_file_list()?;
    println!("{}", project);

    // Parse all ERL files and their included includes
    // if let Err(e) = ErlParseStage::run_parse_stage(&project) {
    //     erl_fatal_icerror(e);
    // }
    Ok(())
}