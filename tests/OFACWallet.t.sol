// Copyright 2024 RISC Zero, Inc.
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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {RiscZeroCheats} from "risc0/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {OFACWallet} from "../contracts/OFACWallet.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.


contract OFACWalletTest is RiscZeroCheats, Test {
    OFACWallet public ofacWallet;

    event FundsSent(address dest, uint256 value);

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        bytes32 sanctionedListHash = 0xe8562c50afc459571ee739a12c4a26c50a1a6aaf0fff5d9b0ea55e8454c43555;
        ofacWallet = new OFACWallet(sanctionedListHash, verifier);
        assertEq(ofacWallet.owner(), address(this));
        uint256 amount = 1 ether;
        vm.deal(address(ofacWallet), amount);
        assertEq(address(ofacWallet).balance, amount);
    }

    function test_SendToNotSanctioned() public {
        address payable dest = payable(0x1111111111111111111111111111111111111111);
        uint256 value = 0.1 ether;
        bytes memory fileContent = bytes(vm.readFile("./sdn_mini.xml"));
        (bytes memory journal, bytes32 post_state_digest, bytes memory seal) =
            prove(Elf.IS_NOT_0FAC_SANCTIONED_PATH, abi.encodePacked(dest, fileContent));
    
        address payable extractedAddress;
        assembly {
            // Load the first 20 bytes of the `journal` directly.
            // Since `journal` is a dynamically-sized bytes array, it has a length prefix.
            // The data starts at the 32 bytes offset due to this prefix.
            // `mload(add(journal, 32))` reads the first 20 bytes after this prefix, which is the address.
            extractedAddress := mload(add(journal, 40))
        }

        vm.expectEmit(true, true, false, true);
        emit FundsSent(dest, value);

        ofacWallet.transferFunds(extractedAddress, value, post_state_digest, seal);
    }
}
