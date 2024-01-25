/* Copyright (C) 2024 Mincraft-essenetials
* This program is free software: you can redistribute it and/or modify it
* under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or (at your
* option) any later version.

* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
* FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public
* License for more details.

* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use base64::Engine;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789\
-.~_";

pub fn verifier(size: usize) -> Vec<u8> {
    assert!(
        (43..=128).contains(&size),
        "Size must be between 43 and 128"
    );

    let mut rng = thread_rng();

    (0..size)
        .map(|_| {
            let i = rng.gen_range(0..CHARS.len());
            CHARS[i]
        })
        .collect()
}

fn url_encode(input: &[u8]) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(input);
    b64.chars()
        .filter_map(|c| match c {
            '=' => None,
            '+' => Some('-'),
            '/' => Some('_'),
            x => Some(x),
        })
        .collect()
}

pub fn code_challenge(code_verifier: &[u8]) -> String {
    let mut sha = Sha256::new();
    sha.update(code_verifier);
    let result = sha.finalize();
    url_encode(&result[..])
}
