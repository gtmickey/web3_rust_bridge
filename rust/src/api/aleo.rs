use std::str::FromStr;

use anyhow::Context;
use rand::rngs::StdRng;
use rand::SeedableRng;
use snarkvm_circuit::prelude::PrimeField;
use snarkvm_console::network::Testnet3;
use snarkvm_console::program::{Environment, FromBytes, Literal, ToBytes};

use crate::aleo::AleoAPIClient;
use crate::aleo::delegate::*;
use crate::api::CurrentNetwork;

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_from_seed(seed: Vec<u8>) -> String {
    let seed: [u8; 32] = seed
        .try_into().unwrap();
    let field = <CurrentNetwork as Environment>::Field::from_bytes_le_mod_order(&seed);
    let reader = &*field.to_bytes_le().unwrap();
    let seed = FromBytes::read_le(reader);
    return PrivateKeyNative::try_from(seed.unwrap()).unwrap().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_to_view_key(private_key: String) -> String {
    let private_key = PrivateKeyNative::from_str(&private_key).unwrap();
    let view_key = ViewKeyNative::try_from(private_key).unwrap();
    return view_key.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_to_address(private_key: String) -> String {
    let private_key = PrivateKeyNative::from_str(&private_key).unwrap();
    let address = AddressNative::try_from(private_key).unwrap();
    return address.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn sign_message(message_bytes: Vec<u8>, private_key: String) -> String {
    let private_key = PrivateKeyNative::from_str(&private_key).unwrap();
    return private_key.sign_bytes(&message_bytes, &mut StdRng::from_entropy()).unwrap().to_string();
}


pub fn generate_public_transfer_delegate_data(private_key: &String,
                                amount_credits: f64,
                                recipient: String,
                                fee_credits: f64, ) -> Vec<String> {
    let private_key_native = PrivateKeyNative::from_str(&private_key).unwrap();
    let result = Delegate::delegate_transfer_public(&private_key_native, amount_credits, &recipient, fee_credits);
    return result;
}

pub fn get_public_balance(url: String, network_id: String, address: String) -> f64 {
    let client = if url.is_empty() {
        AleoAPIClient::testnet3()
    } else {
        AleoAPIClient::new(&url, &network_id).unwrap()
    };
    let public_address_literal = Literal::<Testnet3>::from_str(&address).unwrap();
    let value = client.get_mapping_value("credits.aleo", "account", public_address_literal);

    let value: u64 = value.unwrap().to_string()
        .trim_end_matches("u64")
        .parse()
        .expect("Failed to parse string as u64");
    let result = value as f64 / 1_000_000.0;
    return result;
}


mod test {
    use crate::api::aleo::*;

    #[test]
    fn test_private_key_from_seed() {
        let bytes = hex::decode("6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd").unwrap();
        let private_key = private_key_from_seed(bytes);
        println!("private_key: {}", private_key);
    }

    #[test]
    fn test_private_key_to_view_key_and_address() {
        let bytes = hex::decode("6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd").unwrap();
        let private_key = private_key_from_seed(bytes);

        let view_key = private_key_to_view_key(private_key.clone());
        println!("view_key: {}", view_key);

        let address = private_key_to_address(private_key.clone());
        println!("address: {}", address);
    }

    #[test]
    fn test_sign_message() {
        let bytes = hex::decode("6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd").unwrap();
        let private_key = private_key_from_seed(bytes);

        let sign_result = sign_message(Vec::new(), private_key);
        println!("sign_result: {}", sign_result);
    }


    #[test]
    fn test_delegate_transfer_public() {
        let bytes = hex::decode("6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd").unwrap();
        let private_key = PrivateKeyNative::from_str(&private_key_from_seed(bytes)).unwrap();
        let result = Delegate::delegate_transfer_public(&private_key, 0.1, "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x", 0.28);

        println!("result {:?}", result);
    }

    #[test]
    fn test_get_public_balance() {
        let balance = get_public_balance("".to_string(), "".to_string(), "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x".to_string());
        println!("balance {:?}", balance);

        let balance = get_public_balance("https://api.explorer.aleo.org/v1".to_string(), "testnet3".to_string(), "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x".to_string());
        println!("balance {:?}", balance);
    }
}