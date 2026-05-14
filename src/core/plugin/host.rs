use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("wasm error: {0}")]
    Wasm(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "plugins")]
mod wasm_host {
    use super::*;
    use wasmtime::{Engine, Func, Linker, Module, Store, TypedFunc};

    pub struct WasmPlugin {
        name: String,
        engine: Engine,
        module: Module,
        process_fn: Option<TypedFunc<(i32, i32), i32>>,
    }

    impl WasmPlugin {
        pub fn load(path: &Path) -> Result<Self, PluginError> {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let wasm_bytes = std::fs::read(path)?;
            let engine = Engine::default();
            let module =
                Module::new(&engine, &wasm_bytes).map_err(|e| PluginError::Wasm(e.to_string()))?;

            Ok(Self {
                name,
                engine,
                module,
                process_fn: None,
            })
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn process(&mut self, input: &str) -> Result<String, PluginError> {
            let mut store = Store::new(&self.engine, self.name.clone());
            let linker = Linker::new(&self.engine);

            linker
                .func_wrap("env", "log", |msg_ptr: i32, msg_len: i32| {
                    println!("[plugin log] ptr={}, len={}", msg_ptr, msg_len);
                })
                .map_err(|e| PluginError::Wasm(e.to_string()))?;

            let instance = linker
                .instantiate(&mut store, &self.module)
                .map_err(|e| PluginError::Wasm(e.to_string()))?;

            let process = instance
                .get_typed_func::<(i32, i32), i32>(&mut store, "process")
                .map_err(|e| PluginError::Wasm(e.to_string()))?;

            let _result = process
                .call(&mut store, (0, input.len() as i32))
                .map_err(|e| PluginError::Wasm(e.to_string()))?;

            Ok(String::new())
        }
    }
}

#[cfg(not(feature = "plugins"))]
mod wasm_host {
    use super::*;

    pub struct WasmPlugin {
        name: String,
    }

    impl WasmPlugin {
        pub fn load(path: &Path) -> Result<Self, PluginError> {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            Ok(Self { name })
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn process(&mut self, _input: &str) -> Result<String, PluginError> {
            Ok(String::new())
        }
    }
}

pub use wasm_host::WasmPlugin;

pub struct PluginManager {
    plugins: HashMap<String, WasmPlugin>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn load_from_directory(&mut self, dir: &Path) -> Result<Vec<String>, PluginError> {
        let mut loaded = Vec::new();
        if !dir.exists() {
            return Ok(loaded);
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                let plugin = WasmPlugin::load(&path)?;
                let name = plugin.name().to_string();
                self.plugins.insert(name.clone(), plugin);
                loaded.push(name);
            }
        }
        Ok(loaded)
    }

    pub fn get(&self, name: &str) -> Option<&WasmPlugin> {
        self.plugins.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut WasmPlugin> {
        self.plugins.get_mut(name)
    }

    pub fn process_all(&mut self, input: &str) -> Vec<(String, String)> {
        let mut results = Vec::new();
        for (name, plugin) in &mut self.plugins {
            if let Ok(output) = plugin.process(input) {
                results.push((name.clone(), output));
            }
        }
        results
    }

    pub fn list(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_plugin_manager() {
        let mgr = PluginManager::new();
        assert!(mgr.list().is_empty());
    }

    #[test]
    fn test_load_nonexistent_directory() {
        let mut mgr = PluginManager::new();
        let loaded = mgr
            .load_from_directory(Path::new("/nonexistent/plugins"))
            .unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_process_all_empty() {
        let mut mgr = PluginManager::new();
        let results = mgr.process_all("test input");
        assert!(results.is_empty());
    }
}
