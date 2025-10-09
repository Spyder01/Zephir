use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ApplicationConfig {
    #[serde(default)]
    pub entry: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum ArtifactType {
    #[default]
    NATIVE,
    WASM,
    LUA,
} 

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ArtifactConfig {
    pub packagePath: String,

    #[serde(default)]
    pub artifactType: ArtifactType,
}

fn default_memory() -> u64 {
    128*1024_u64.pow(2)
}

fn default_storage() -> u64 {
    512*1024_u64.pow(2)
}

fn default_cpu_time() -> u64 {
    10_u64
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResourceConfig {
    
    #[serde(default="default_memory")]
    pub memory: u64,

    #[serde(default="default_storage")]
    pub storage: u64,

    #[serde(default="default_cpu_time")]
    pub cpuLimit: u64
}



#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FunctionConfig {
    #[serde(default)]
    pub bundle: ArtifactConfig,
    pub app: ApplicationConfig,

    #[serde(default)]
    pub resources: ResourceConfig,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    pub sandbox: Option<String>,
    pub cache: Option<String>,
}

impl StorageConfig {
    pub fn sane_defaults() -> Self {
        Self {
            sandbox: Some("zephir-sandbox/".to_string()),
            cache: Some("zephir-cache/".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(default)]
    pub toFile: bool,
    pub filePath: Option<String>,

    #[serde(default)]
    pub toStdout: bool,
    pub prefix: Option<String>,

    #[serde(default)]
    pub debugEnabled: bool,
}

fn default_name() -> String {
    String::from("zephir-function")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZephirConfig {
    #[serde(default="default_name")]
    pub name: String,

    #[serde(default)]
    pub function: FunctionConfig, 

    pub storage: Option<StorageConfig>,
    pub logConfig: Option<LogConfig>,
}

impl ZephirConfig {

    pub fn sane_defaults() -> Self {
        ZephirConfig {
            name: default_name(),
            function: FunctionConfig {
                app: ApplicationConfig {
                    entry: "./zephir-function".to_string(),
                },
                bundle: ArtifactConfig {
                    packagePath: "function.zephir".to_string(),
                    artifactType: ArtifactType::NATIVE,
                },
                resources: ResourceConfig {
                    memory: default_memory(),
                    storage: default_storage(),
                    cpuLimit: default_cpu_time(),
                },
            },
            storage: Some(StorageConfig::sane_defaults()),
            logConfig: Some(LogConfig {
                toFile: false,
                filePath: None,
                toStdout: true,
                prefix: Some("[Zephir]".to_string()),
                debugEnabled: false,
            }),
        }
    }
}
