// SPDX-License-Identifier: Apache-2.0
// This file is part of Verkel rust crate.
//
// Copyright (c) 2021 Wei Tang.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ark_ec::PairingEngine;
use ark_poly_commit::kzg10::Commitment;

pub trait Parameters {
    /// Data type for leaf node.
    type LeafData;
    /// Additional data type for branch node.
    type BranchAdditionalData;
    /// Pairing engine.
    type PairingEngine: PairingEngine;
}

pub enum Node<P: Parameters, const WIDTH: usize> {
    Leaf(P::LeafData),
    Branch(Box<[Node<P, WIDTH>; WIDTH]>, Commitment<P::PairingEngine>, P::BranchAdditionalData),
}
