use dotenv::dotenv;
use envy::Error;
use serde::Deserialize;

// Parameters of the environment file
pub enum EnvParams {
    Mnemonic,
}

#[derive(Deserialize, Debug)]
pub struct PaynymEnv {
    mnemonic: String,
}

impl PaynymEnv {

    pub fn initialise() -> Result<Self, String> {
        // Load the environment variables from the ".env" file.
        dotenv().ok();
        
        match envy::from_env::<PaynymEnv>() {
            Ok(env) => Ok(env),
            Err(e) => {
                let mut message: String = String::from("the .env file does not exist");
                println!("{:#?}", e);
                if let Error::MissingValue(envy_message) = e {
                    message = format!("ENV: Missing param {:#}", envy_message);
                } else if let Error::Custom(custom_message) = e {
                    message = format!("{:#}", custom_message);
                }
                panic!("{}", message)
            }
        }
    }

    pub fn get(&self, name: EnvParams) -> String {
        let param = match name {
            EnvParams::Mnemonic => &self.mnemonic
        };
        param.clone()
    }
}