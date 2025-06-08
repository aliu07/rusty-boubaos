use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::RwLock};

use crate::errors::{self, BadCommandError};

const VAR_TABLE_SIZE: usize = 10;

pub struct VariableTable {
    table: RwLock<HashMap<String, String>>,
}

impl VariableTable {
    pub fn set_value(&self, variable: &str, value: &str) -> Result<(), BadCommandError> {
        let mut table = self
            .table
            .write()
            .map_err(|err| return BadCommandError::VariableTableError(err.to_string()))?;

        // Check capacity
        if table.len() == VAR_TABLE_SIZE {
            return Err(errors::variable_table_full());
        }

        table
            .entry(String::from(variable))
            .and_modify(|val| {
                *val = String::from(value);
            })
            .or_insert(String::from(value));

        Ok(())
    }

    pub fn get_value(&self, variable: &str) -> Result<String, BadCommandError> {
        let table = self
            .table
            .read()
            .map_err(|err| return BadCommandError::VariableTableError(err.to_string()))?;

        table
            .get(variable)
            .cloned()
            .ok_or_else(|| BadCommandError::VariableDoesNotExist(String::from(variable)))
    }
}

// Singleton lazy access
pub static VARIABLE_TABLE: Lazy<VariableTable> = Lazy::new(|| VariableTable {
    table: RwLock::new(HashMap::with_capacity(VAR_TABLE_SIZE)),
});
