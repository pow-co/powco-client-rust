
use std::env;

mod powco {

    use sv::address::addr_decode;
    use sv::network::Network::Mainnet;
    use bitcoin::util::key::{PrivateKey, PublicKey};
    use bitcoin::util::address::{Address};
    use bitcoin::network::constants::Network;
    use secp256k1::{Secp256k1};
    use rand::rngs::OsRng;
    //use secp256k1::rand::rngs::OsRng;
    use getrandom::getrandom;

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

    impl Client {
        pub fn has_valid_keys(&self) -> bool {
            self.keys.validate()
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
}

