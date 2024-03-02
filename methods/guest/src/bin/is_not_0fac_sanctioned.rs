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

use json::parse;
use json_core::Inputs;
use json_core::Outputs;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

use roxmltree::Document;
use std::fs;

fn main() {
    let inputs: Inputs = env::read();

    let ofac_list: String = inputs.ofac_list;
    let checked_address: String = inputs.checked_address;

    let sha = *Impl::hash_bytes(&ofac_list.as_bytes());


    let doc = Document::parse(&ofac_list).expect("Failed to parse XML");
    let ns = "http://www.un.org/sanctions/1.0";

    let mut found = false;

    for dps in doc.descendants().filter(|n| n.has_tag_name(("DistinctParties", ns))) {
        for dp in dps.descendants().filter(|n| n.has_tag_name(("DistinctParty", ns))) {
            for p in dp.descendants().filter(|n| n.has_tag_name(("Profile", ns))) {
                for feature in p.descendants().filter(|n| n.has_tag_name(("Feature", ns))) {
                    // Find the VersionDetail element within each Feature
                    let version_detail = feature.descendants().find(|n| n.has_tag_name(("VersionDetail", ns)));
                    if let Some(version_detail) = version_detail {
                        if version_detail.text().unwrap_or_default().trim() == checked_address {
                            found = true;
                            break;
                        }
                    }
                }
                if found { break; }
            }
            if found { break; }
        }
        if found { break; }
    }

    let proven_val = if found { true } else { false };
    let out = Outputs {
        is_0fac_sanctioned: proven_val,
        ofac_list_hash: sha,
    };
    env::commit(&out);
}
