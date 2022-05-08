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

use aleo_account::{Account as AccountNative, PrivateKey};

use rand::{rngs::StdRng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    pub(crate) account: AccountNative,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let rng = &mut StdRng::from_entropy();
        Self {
            account: AccountNative::new(rng),
        }
    }

    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();

        Self {
            account: AccountNative::from(private_key),
        }
    }

    #[wasm_bindgen]
    pub fn from_seed(seed: u64) -> Self {
        console_error_panic_hook::set_once();

        Self {
            account: AccountNative::new(&mut ChaChaRng::seed_from_u64(seed)),
        }
    }

    #[wasm_bindgen]
    pub fn to_private_key(&self) -> String {
        self.account.private_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_view_key(&self) -> String {
        self.account.view_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_address(&self) -> String {
        self.account.address().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkpCfr2a7zoqz35qnJuyAMGzaGupqQTMqMD7skj5eP1Zjck";
    const ALEO_VIEW_KEY: &str = "AViewKey1mMm5Tjekqw7jVjAFapyFiryGZmzWr5Qm3cKq7EY5nTGC";
    const ALEO_ADDRESS: &str = "aleo1fmyklartl6clae8znsqu2ptu9nqtyv9840eujnz9pr5u9vsh358ssl2zrj";
    const ALEO_SEED: u64 = 12345;

    #[wasm_bindgen_test]
    pub fn from_private_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_PRIVATE_KEY, account.account.private_key().to_string());
        assert_eq!(ALEO_PRIVATE_KEY, account.account.private_key().to_string());

        println!("{} == {}", ALEO_VIEW_KEY, account.account.view_key().to_string());
        assert_eq!(ALEO_VIEW_KEY, account.account.view_key().to_string());

        println!("{} == {}", ALEO_ADDRESS, account.account.address().to_string());
        assert_eq!(ALEO_ADDRESS, account.account.address().to_string());
    }

    #[wasm_bindgen_test]
    pub fn from_seed_test() {
        let account = Account::from_seed(ALEO_SEED);

        println!("{} == {}", ALEO_PRIVATE_KEY, account.account.private_key().to_string());
        assert_eq!(ALEO_PRIVATE_KEY, account.account.private_key().to_string());

        println!("{} == {}", ALEO_VIEW_KEY, account.account.view_key().to_string());
        assert_eq!(ALEO_VIEW_KEY, account.account.view_key().to_string());

        println!("{} == {}", ALEO_ADDRESS, account.account.address().to_string());
        assert_eq!(ALEO_ADDRESS, account.account.address().to_string());
    }

    #[wasm_bindgen_test]
    pub fn to_private_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_PRIVATE_KEY, account.to_private_key());
        assert_eq!(ALEO_PRIVATE_KEY, account.to_private_key());
    }

    #[wasm_bindgen_test]
    pub fn to_view_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_VIEW_KEY, account.to_view_key());
        assert_eq!(ALEO_VIEW_KEY, account.to_view_key());
    }

    #[wasm_bindgen_test]
    pub fn to_address_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_ADDRESS, account.to_address());
        assert_eq!(ALEO_ADDRESS, account.to_address());
    }
}
