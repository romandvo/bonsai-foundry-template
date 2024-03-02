// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, hex::FromHex};
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_address_is_not_sanctioned() {
        let non_sanctioned_address = Address::from_hex("0x1111111111111111111111111111111111111111").unwrap();

        let env = ExecutorEnv::builder()
            .write_slice(&non_sanctioned_address.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();

        let dest = Address::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(dest, non_sanctioned_address);
    }

    #[test]
    #[should_panic(expected = "address is sanctioned")]
    fn rejects_sanctioned_address() {
        let sanctioned_address = Address::from_hex("0x01e2919679362dFBC9ee1644Ba9C6da6D6245BB1").unwrap();

        let env = ExecutorEnv::builder()
            .write_slice(&sanctioned_address.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
    }
}
