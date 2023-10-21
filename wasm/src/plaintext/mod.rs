// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use std::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::*;
use snarkvm_console::prelude::{ToBits, Network};
use crate::types::native::{CurrentNetwork, PlaintextNative};
use crate::types::Field;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Plaintext(PlaintextNative);

#[wasm_bindgen]
impl Plaintext {
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(plaintext: &str) -> Result<Plaintext, String> {
        Self::from_str(plaintext).map_err(|_| "The plaintext string provided was invalid".into())
    }

    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = hashBhp256)]
    pub fn hash_bhp256(&self) -> Result<Field, String> {
        CurrentNetwork::hash_bhp256(&self.0.to_bits_le())
            .map(|field| Field::from(field))
            .map_err(|e| e.to_string())
    }
}

impl From<PlaintextNative> for Plaintext {
    fn from(record: PlaintextNative) -> Self {
        Self(record)
    }
}

impl FromStr for Plaintext {
    type Err = anyhow::Error;

    fn from_str(plaintext: &str) -> Result<Self, Self::Err> {
        Ok(Self(PlaintextNative::from_str(plaintext)?))
    }
}

impl Deref for Plaintext {
    type Target = PlaintextNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
