use crate::errors::Result;
use crate::schema::{Config, Table};
use crate::GenerateOption;
use rust_format::{Formatter, RustFmt};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
mod struct_def;

pub fn run(config: &Config, opt: &GenerateOption) -> Result<()> {
    //match sure we are working in a valid project
    //super::validate_project_path(&opt.project_dir)?;

    let tables: Vec<_> = config
        .tables
        .iter()
        .filter(|x| opt.table.is_none() || opt.table == Some(x.name.to_string()))
        .collect();

    for table in tables {
        let path = model_path(&opt.project_dir, &table);
        fs::create_dir_all(&path)?;
        init_mod_file(&path)?;
        init_customizations(&path)?;
        init_sql(&path)?;
        struct_def::generate(&path, &table)?;
    }

    Ok(())
}

fn init_mod_file(path: &PathBuf) -> Result<()> {
    let mut path = PathBuf::from(path);
    path.push("mod.rs");
    if path.exists() {
        return Ok(());
    }

    let code = quote::quote! {
        mod customizations;
        mod definition;
        mod sql;
        pub use customizations::*;
        pub use definition::*;
        pub use sql::*;
    };

    let mut file = File::create(path)?;
    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    file.write_all(formated.as_bytes())?;
    Ok(())
}

fn init_customizations(mod_path: &PathBuf) -> Result<()> {
    let mut path = PathBuf::from(mod_path);
    path.push("customizations.rs");
    if path.exists() {
        return Ok(());
    }
    let mut file = File::create(path)?;
    file.write_all(&[])?;
    Ok(())
}

fn init_sql(mod_path: &PathBuf) -> Result<()> {
    let mut path = PathBuf::from(mod_path);
    path.push("sql.rs");
    if path.exists() {
        return Ok(());
    }
    let mut file = File::create(path)?;
    file.write_all(&[])?;
    Ok(())
}

fn model_path(project_dir: &PathBuf, table: &Table) -> PathBuf {
    let mut path = PathBuf::from(project_dir);
    path.push("src");
    path.push("models");
    path.push(table.module_name());
    path
}
