use super::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use swagger::{PassiveChecks, PassiveScanType};

const TOKEN_FILE: &str = ".cherrybomb/token.txt";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub scan_type: PassiveScanType,
    //for now description
    pub alerts_ignore: Vec<String>,
    pub fail_on_info: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            scan_type: PassiveScanType::Full,
            alerts_ignore: vec![],
            fail_on_info: true,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum StrScanType {
    Full(String),
    Partial(Vec<String>),
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigStr {
    scan_type: StrScanType,
    pub alerts_ignore: Vec<String>,
    pub fail_on_info: bool,
}
impl Default for ConfigStr {
    fn default() -> Self {
        ConfigStr {
            scan_type: StrScanType::Full("Full".to_string()),
            alerts_ignore: vec![],
            fail_on_info: true,
        }
    }
}
impl Config {
    fn from_conf_str(conf_str: ConfigStr) -> Config {
        let scan_type = match conf_str.scan_type {
            StrScanType::Full(_) => PassiveScanType::Full,
            StrScanType::Partial(vec) => PassiveScanType::Partial(
                vec.iter()
                    .filter_map(|check| {
                        let c = PassiveChecks::from_string(check);
                        if c.is_none() {
                            println!(
                                "Check name: {check} does not exist, the config will load without it."
                            );
                        }
                        c
                    })
                    .collect(),
            ),
        };
        Config {
            scan_type,
            alerts_ignore: conf_str.alerts_ignore,
            fail_on_info: conf_str.fail_on_info,
        }
    }
    pub fn from_file(file: &str) -> Option<Config> {
        let mut filename = dirs::home_dir().unwrap();
        let dir = dirs::home_dir().unwrap();
        filename.push(file);
        let mut file = match File::open(&mut filename) {
            Ok(f) => f,
            Err(_) => {
                let mut f = if let Ok(ff) = File::create(&filename) {
                    ff
                } else {
                    match std::fs::create_dir(dir) {
                        Ok(_) => {
                            if let Ok(ff) = File::create(filename) {
                                ff
                            } else {
                                println!("Could not create config file, please make sure the cherrybomb dir is set");
                                return None;
                            }
                        }
                        Err(_) => {
                            println!("Could not create config file, please make sure the cherrybomb dir is set");
                            return None;
                        }
                    }
                };
                f.write_all(
                    serde_json::to_string(&Config::default())
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
                return Some(Config::default());
            }
        };
        let mut file_data = String::new();
        match file.read_to_string(&mut file_data) {
            Ok(_) => (),
            Err(_) => {
                print_err("Could not read data from config file");
                return None;
            }
        };
        if let Ok(conf) = serde_json::from_str::<ConfigStr>(&file_data) {
            Some(Config::from_conf_str(conf))
        } else {
            println!(
                "Config does not match format, go to our docs on github for further explanation"
            );
            None
        }
    }
}
async fn create_token(filename: &Path, dir: &Path) -> bool {
    use uuid::Uuid;
    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(_) => match std::fs::create_dir(dir) {
            Ok(_) => match File::create(filename) {
                Ok(f) => f,
                Err(_) => {
                    return false;
                }
            },
            Err(_) => {
                return false;
            }
        },
    };
    file.write_all(Uuid::new_v4().to_string().as_bytes())
        .unwrap_or_default();
    true
}
async fn get_token() -> String {
    let mut filename = dirs::home_dir().unwrap();
    filename.push(TOKEN_FILE);
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".cherrybomb");
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => {
            if create_token(&filename, &dir).await {
                match File::open(&filename) {
                    Ok(f) => f,
                    Err(_) => {
                        return String::new();
                    }
                }
            } else {
                return String::new();
            }
        }
    };
    let mut token = String::new();
    match file.read_to_string(&mut token) {
        Ok(_) => (),
        Err(_) => {
            return String::new();
        }
    }
    token
}
pub async fn try_send_telemetry(no_tel: Option<bool>, action: &str) {
    if let Some(t) = no_tel {
        if t {
            return;
        }
    }
    let token = get_token().await;
    let client = reqwest::Client::new();
    let _ = client
        .post("https://cherrybomb.blstsecurity.com/tel")
        .body(format!(
            "{{\"client_token\":\"{token}\",\"event\":\"{action}\"}}"
        ))
        .send()
        .await;
}
