use std::ops::Range;
use snarkvm_console::account::{Field, PrivateKey, ViewKey};
use snarkvm_console::program::{Ciphertext, Identifier, Plaintext, ProgramID, Record, Value};
use super::*;

#[cfg(not(feature = "async"))]
#[allow(clippy::type_complexity)]
impl<N: Network> AleoAPIClient<N> {
    /// Get the current value of a mapping given a specific program, mapping name, and mapping key
    pub fn get_mapping_value(
        &self,
        program_id: impl TryInto<ProgramID<N>>,
        mapping_name: impl TryInto<Identifier<N>>,
        key: impl TryInto<Plaintext<N>>,
    ) -> Result<Value<N>> {
        // Prepare the program ID.
        let program_id = program_id.try_into().map_err(|_| anyhow!("Invalid program ID"))?;
        // Prepare the mapping name.
        let mapping_name = mapping_name.try_into().map_err(|_| anyhow!("Invalid mapping name"))?;
        // Prepare the key.
        let key = key.try_into().map_err(|_| anyhow!("Invalid key"))?;
        // Perform the request.
        let url = format!("{}/{}/program/{program_id}/mapping/{mapping_name}/{key}", self.base_url, self.network_id);
        match self.client.get(&url).call()?.into_json() {
            Ok(transition_id) => Ok(transition_id),
            Err(error) => bail!("Failed to parse transition ID: {error}"),
        }
    }
}