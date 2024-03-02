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

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be even.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on Bonsai.
contract OFACWallet {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.IS_NOT_0FAC_SANCTIONED_ID;

    address public owner;
    bytes32 public sanctionListHash;

    event FundsSent(address dest, uint value);


    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(bytes32 _sanctionListHash, IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        owner = msg.sender;
        sanctionListHash = _sanctionListHash;

    }



    /// @notice Send dest ETH amount. Requires a RISC Zero proof that the dest is not an OFAC sanctioned address.
    function transferFunds(address payable dest, uint256 amount, bytes32 postStateDigest,
     bytes calldata seal) external onlyOwner {
        require(address(this).balance >= amount, "Insufficient balance");
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(dest, sanctionListHash);
        require(verifier.verify(seal, imageId, postStateDigest, sha256(journal)));
        dest.transfer(amount);
        emit FundsSent(dest, amount);
    }

    receive() external payable {}

    fallback() external payable {}
}
