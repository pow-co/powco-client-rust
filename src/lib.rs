
mod powco {

    use sv::address::addr_decode;
    use sv::network::Network::Mainnet;
    use bitcoin::util::key::{PrivateKey, PublicKey};
    use bitcoin::util::address::{Address};
    use secp256k1::{Secp256k1};
    use reqwest;
    use serde::{Serialize, Deserialize, Deserializer};
    use std::fmt::Display;
    use std::str::FromStr;
    //use serde_aux::prelude::*;

    pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + serde::Deserialize<'de>,
        <T as FromStr>::Err: Display,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt<T> {
            String(String),
            Number(T),
        }

        match StringOrInt::<T>::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
            StringOrInt::Number(i) => Ok(i),
        }
    }

    pub struct Keys {
        pub identifier: String,
        pub secret: String,
    }

    impl Keys {
        pub fn generate() -> Keys {
                
            let secp = Secp256k1::new();

            let mut rng = rand::rngs::OsRng::new().expect("OsRng");

            let (secret_key, public_key) = secp.generate_keypair(&mut rng);

            let serialized_key = secret_key.serialize_secret();

            let serialized_public_key = public_key.serialize();

            let network = bitcoin::network::constants::Network::Bitcoin;

            let private_key = PrivateKey::from_slice(&serialized_key, network).unwrap();

            let pubkey = PublicKey::from_slice(&serialized_public_key).unwrap();

            let address = Address::p2pkh(&pubkey, network);

            Keys {
                identifier: address.to_string(),
                secret: private_key.to_wif(),
            }
        }

        pub fn validate(&self) -> bool {

           if self.validate_identifier() {

                if self.validate_secret() {

                    true

                } else {

                    false
                }

           } else {

               false
           }
        }

        fn validate_identifier(&self) -> bool {
            match addr_decode(&self.identifier, Mainnet) {
                Ok((_, _)) => {
                    true
                },
                Err(_) => {
                    false
                }
            }
        }

        fn validate_secret(&self) -> bool {
            match PrivateKey::from_wif(&self.secret) {
                Ok(_) => {
                    true
                },
                Err(_) => {
                    false
                }
            }
        }
    }

    pub struct Client {
        pub keys: Keys,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Job {
        pub content: String,
        pub category: String,
        pub tag: String,
        pub txid: String,
        pub vout: u32,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub difficulty: f32,
        pub value: f32,
        pub spent: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ListJobsAPIResponse {
        jobs: Vec<Job>,
    }

    impl Client {

        pub fn has_valid_keys(&self) -> bool {
            self.keys.validate()
        }

        pub async fn list_available_jobs(&self) -> Vec<Job> {
            let client = reqwest::Client::new();
            let response = client
                .get("https://pow.co/api/v1/jobs")
                .send()
                .await
                .unwrap();

            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<ListJobsAPIResponse>().await {
                        Ok(parsed) => {
                            println!("{}", parsed.jobs.len());
                            parsed.jobs
                        },
                        Err(_) => {
                            println!("Error getting jobs from API");
                            let result: Vec<Job> = Vec::new();
                            result
                        }
                    }
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    let result: Vec<Job> = Vec::new();
                    result
                }
                other => {
                    panic!("Something unexpected happened when calling pow.co API {:?}", other);
                }

            }


        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generating_new_api_keys() {
        
        let keys = powco::Keys::generate();

        assert!(keys.validate())

    }

    #[test]
    fn client_accepts_valid_keys() {

        let keys = powco::Keys {
            identifier: "1Crbut9jpa382XQDtz4Dwwetys3B1Vr1My".to_string(),
            secret: "L1ZnxCq4rJY4bzpznHJR5B4upmuiZ6Bx4Fs3RGS3kVg1xrSq4zGH".to_string(),
        };

        let client = powco::Client {
            keys: keys
        };
        
        assert!(client.has_valid_keys());

    }

    #[test]
    fn client_rejects_invalid_identifier() {

        let keys = powco::Keys {
            identifier: "1Crbut9jpa382XQDtz4".to_string(),
            secret: "L1ZnxCq4rJY4bzpznHJR5B4upmuiZ6Bx4Fs3RGS3kVg1xrSq4zGH".to_string(),
        };

        let client = powco::Client {
            keys: keys
        };
        
        assert!(!client.has_valid_keys());

    }


    #[test]
    fn client_rejects_invalid_secret() {

        let keys = powco::Keys {
            identifier: "1Crbut9jpa382XQDtz4Dwwetys3B1Vr1My".to_string(),
            secret: "L1ZnxCq4rJY4bzPzn".to_string(),
        };

        let client = powco::Client {
            keys: keys
        };
        
        assert!(!client.has_valid_keys());

    }

    #[tokio::test]
    async fn client_gets_list_of_available_jobs() {

        let keys = powco::Keys::generate();

        let client = powco::Client {
            keys: keys
        };

        let jobs = client.list_available_jobs().await;
        assert!(jobs.len() > 0);

        let job = jobs.get(0).unwrap();

        assert!(job.difficulty > 0.0);
        assert!(job.value > 0.0);

    }
}

