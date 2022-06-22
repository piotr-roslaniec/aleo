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

use aleo_account::{Address as AddressNative, PrivateKey, ViewKey};

use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Address {
    pub(crate) address: AddressNative,
}

#[wasm_bindgen]
impl Address {
    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();
        let address = AddressNative::from(&private_key);
        Self { address }
    }

    #[wasm_bindgen]
    pub fn from_view_key(view_key: &str) -> Self {
        let view_key = ViewKey::from_str(view_key).unwrap();
        let address = AddressNative::from_view_key(&view_key);
        Self { address }
    }

    #[wasm_bindgen]
    pub fn from_string(address: &str) -> Self {
        let address = AddressNative::from_str(address).unwrap();
        Self { address }
    }

    /// Verify a signature signed by the view key
    /// Returns `true` if the signature is verified correctly. Otherwise, returns `false`.
    #[wasm_bindgen]
    pub fn verify_signature(&self, message: &str, signature: Vec<u8>) -> bool {
        console_error_panic_hook::set_once();
        
        self.address
            .verify_signature_bytes(&message.as_bytes(), &signature)
            .unwrap()
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.address.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp4md8AREpQMoEmmVG8kAp8qKpgT95o6upA9ZzL2yHYUMM";
    const ALEO_VIEW_KEY: &str = "AViewKey1oUHFc3G2ioQ1w2uPne162XPpWJ3gbWizmrqiSjmpuyaP";
    const ALEO_ADDRESS: &str = "aleo1lakxjm562uz3ekufmtnma56m6k4utmg82tgv55zt4tymn8e9v5fq6gsgun";

    #[wasm_bindgen_test]
    pub fn from_private_key_test() {
        let address = Address::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_ADDRESS, address.to_string());
        assert_eq!(ALEO_ADDRESS, address.to_string());
    }

    #[wasm_bindgen_test]
    pub fn from_view_key_test() {
        let address = Address::from_view_key(ALEO_VIEW_KEY);

        println!("{} == {}", ALEO_ADDRESS, address.to_string());
        assert_eq!(ALEO_ADDRESS, address.to_string());
    }

    #[wasm_bindgen_test]
    pub fn from_address_string() {
        let address = Address::from_string(ALEO_ADDRESS);

        println!("{} == {}", ALEO_ADDRESS, address.to_string());
        assert_eq!(ALEO_ADDRESS, address.to_string());
    }
}
