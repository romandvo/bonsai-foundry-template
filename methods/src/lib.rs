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

use core::Inputs;
use core::Outputs;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn check_sanction(ofac_list: &str, checked_address: &str) -> Outputs {
    let inputs = Inputs {
        ofac_list: ofac_list.parse().unwrap(),
        checked_address: checked_address.parse().unwrap()
    };
    println!("building env...");
    let env = ExecutorEnv::builder()
        .write(&inputs)
        .unwrap()
        .build()
        .unwrap();

    println!("creating prover...");
    // Obtain the default prover.
    let prover = default_prover();

    println!("proving...");
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
    println!("proved!");

    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, hex::FromHex};
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, ExecutorEnv};
    use crate::check_sanction;

    #[test]
    fn proves_address_is_not_sanctioned()  {
        println!("loading inputs...");
        let ofac_list = include_str!("../../sdn_mini.xml");
        let checked_address = "0xkosher";
        println!("inputs loaded!");

        let concatenated = format!("{}{}", checked_address, ofac_list);

        println!("building env...");
        let env = ExecutorEnv::builder()
            .write_slice(&concatenated.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session = default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
        println!("res {}", session.journal.decode());
    }

    #[test]
    #[should_panic(expected = "address is sanctioned")]
    fn rejects_sanctioned_address() {
        println!("loading inputs...");
        let ofac_list = include_str!("/home/ben/ethdenver/ofac-sanctioned-digital-currency-addresses/sdn_mini.xml");
        let checked_address = "0xb04E030140b30C27bcdfaafFFA98C57d80eDa7B4";
        println!("inputs loaded!");

        let concatenated = format!("{}{}", checked_address, ofac_list);

        println!("building env...");
        let env = ExecutorEnv::builder()
            .write_slice(&concatenated.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session = default_executor().execute(env, super::IS_NOT_0FAC_SANCTIONED_ELF).unwrap();
        println!("res {}", session.journal.decode());
    }
}
