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

// use core::Inputs;
// use core::Outputs;
// use risc0_zkvm::{default_prover, ExecutorEnv};


#[cfg(test)]
mod tests {
    // use alloy_primitives::{Address, hex::FromHex};
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_address_is_not_sanctioned()  {
        println!("loading inputs...");
        let ofac_list = include_str!("../../sdn_mini.xml");
        let checked_address = "kosheraaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        println!("inputs loaded!");

        let concatenated = format!("{}{}", checked_address, ofac_list);

        println!("building env...");
        let env = ExecutorEnv::builder()
            .write_slice(&concatenated.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
    }

    #[test]
    #[should_panic(expected = "address is sanctioned")]
    fn rejects_sanctioned_address() {
        println!("loading inputs...");
        let ofac_list = include_str!("../../sdn_mini.xml");
        let checked_address = "b04E030140b30C27bcdfaafFFA98C57d80eDa7B4";
        println!("inputs loaded!");

        let concatenated = format!("{}{}", checked_address, ofac_list);

        println!("building env...");
        let env = ExecutorEnv::builder()
            .write_slice(&concatenated.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
    }
}
