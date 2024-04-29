use std::ops::Add;
use std::str::FromStr;
use rand::rngs::StdRng;
use rand::SeedableRng;
use snarkvm_circuit::AleoV0;
use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::Address,
};
use snarkvm_console::program::Identifier;
use snarkvm_synthesizer::{Process, Program};

use crate::api::CurrentNetwork;

pub type AddressNative = Address<CurrentNetwork>;
pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
pub type ViewKeyNative = ViewKey<CurrentNetwork>;
pub type ProgramNative = Program<CurrentNetwork>;

pub type ProcessNative = Process<CurrentNetwork>;

pub type IdentifierNative = Identifier<CurrentNetwork>;


pub struct Delegate;


impl Delegate {
    pub fn delegate_transfer_public(
        private_key: &PrivateKeyNative,
        amount_credits: f64,
        recipient: &str,
        fee_credits: f64,
    ) -> Vec<String> {
        let fee_microcredits = (fee_credits * 1_000_000.0) as u64;
        let amount_microcredits = (amount_credits * 1_000_000.0) as u64;

        let program = ProgramNative::credits().unwrap().to_string();
        let rng = &mut StdRng::from_entropy();

        let (transfer_type, inputs) = {
            let mut inputs = Vec::<String>::new();
            inputs.push(recipient.to_string());
            inputs.push(amount_microcredits.to_string().add("u64"));
            ("transfer_public", inputs)
        };


        let mut process_native = ProcessNative::load().map_err(|err| err.to_string()).unwrap();
        let process = &mut process_native;

        let program =
            ProgramNative::from_str(&program).map_err(|_| "The program ID provided was invalid".to_string()).unwrap();

        let function_name = IdentifierNative::from_str(transfer_type)
            .map_err(|_| "The function name provided was invalid".to_string()).unwrap();

        let program_id = program.clone().id().to_string();

        let authorization = process.authorize::<AleoV0, _>(private_key, program_id, function_name, inputs.iter(), rng).unwrap();

        let fee_authorization = process.authorize_fee_public::<AleoV0, _>(private_key, fee_microcredits, 0u64, authorization.clone().to_execution_id().unwrap(), rng).unwrap();
        return vec![authorization.clone().to_string(), program.to_string(), fee_authorization.clone().to_string()];
    }
}
