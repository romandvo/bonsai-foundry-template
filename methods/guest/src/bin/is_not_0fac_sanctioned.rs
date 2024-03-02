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

use std::io::Read;

use alloy_primitives::Address;
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let address = <Address>::abi_decode(&input_bytes, true).unwrap();

    // Run the computation.
    // In this case, asserting that the provided number is not sanctioned.
    assert!(address.to_string() != "0x01e2919679362dFBC9ee1644Ba9C6da6D6245BB1", "address is sanctioned");

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit_slice(address.abi_encode().as_slice());
}
