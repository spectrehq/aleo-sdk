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

use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::native::ProgramIDNative;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramID(ProgramIDNative);

#[wasm_bindgen]
impl ProgramID {
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(program_id: &str) -> Result<ProgramID, String> {
        Ok(Self(ProgramIDNative::from_str(program_id).map_err(|e| e.to_string())?))
    }

    #[wasm_bindgen(js_name = "toAddress")]
    pub fn to_address(&self) -> String {
        self.0.to_address().unwrap().to_string()
    }
}
