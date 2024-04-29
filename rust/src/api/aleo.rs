use std::str::FromStr;
use anyhow::{Context, Result};
use rand::rngs::StdRng;
use rand::SeedableRng;
use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Address, Environment, FromBytes, ToBytes},
};
use crate::api::CurrentNetwork;

use snarkvm_circuit::prelude::PrimeField;


type AddressNative = Address<CurrentNetwork>;
type PrivateKeyNative = PrivateKey<CurrentNetwork>;
type ViewKeyNative = ViewKey<CurrentNetwork>;

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
}