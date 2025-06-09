use once_cell::sync::Lazy;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

use crate::errors::BadCommandError;

pub const ROOT_DIR: &str = "root";

pub struct EnvManager {
    inner: RwLock<Environment>,
}

impl EnvManager {
    pub fn new(env: Environment) -> Self {
        Self {
            inner: RwLock::new(env),
        }
    }

    pub fn get_root_path(&self) -> PathBuf {
        self.inner.read().unwrap().root_path.clone()
    }

    pub fn get_current_path(&self) -> PathBuf {
        self.inner.read().unwrap().current_path.clone()
    }

    pub fn set_current_path(&self, new_path: &PathBuf) {
        self.inner.write().unwrap().current_path = new_path.clone();
    }
}

pub struct Environment {
    root_path: PathBuf,
    current_path: PathBuf,
}

impl Environment {
    pub fn build() -> anyhow::Result<Self> {
        let root_path = Path::new(ROOT_DIR);

        if !root_path.exists() {
            fs::create_dir(root_path).map_err(|_| BadCommandError::CreateDirectoryError)?;
        }

        let root_path = fs::canonicalize(root_path)?;
        let current_path = root_path.clone();

        env::set_current_dir(&current_path).expect("Failed to enter root directory");

        Ok(Environment {
            root_path: root_path,
            current_path: current_path,
        })
    }
}

pub static ENV: Lazy<EnvManager> = Lazy::new(|| {
    let env = Environment::build().expect("Failed to initialize the environment");
    EnvManager::new(env)
});
