// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use aleo_account::{PrivateKey, ViewKey as ViewKeyNative};

use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ViewKey {
    pub(crate) view_key: ViewKeyNative,
}

#[wasm_bindgen]
impl ViewKey {
    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();
        let view_key = ViewKeyNative::from(&private_key);
        Self { view_key }
    }

    #[wasm_bindgen]
    pub fn from_string(view_key: &str) -> Self {
        let view_key = ViewKeyNative::from_str(view_key).unwrap();
        Self { view_key }
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.view_key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp4md8AREpQMoEmmVG8kAp8qKpgT95o6upA9ZzL2yHYUMM";
    const ALEO_VIEW_KEY: &str = "AViewKey1oUHFc3G2ioQ1w2uPne162XPpWJ3gbWizmrqiSjmpuyaP";

    #[wasm_bindgen_test]
    pub fn from_private_key_test() {
        let view_key = ViewKey::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_VIEW_KEY, view_key.to_string());
        assert_eq!(ALEO_VIEW_KEY, view_key.to_string());
    }
}
