/* Copyright © 2018 Jeremy Cline
   Copyright © 2017-2018 Randy Barlow

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.*/
//! # yubibomb
//!
//! ```yubibomb``` returns fake HOTP tokens so you can paste them into IRC.

extern crate rand;

use rand::Rng;


/// Returns an HOTP token.
///
/// # Examples
///
/// ```
/// println!("{}", yubibomb::hotp());
/// ```
pub fn hotp() -> String {
    format!("{:06}", rand::thread_rng().gen_range(0, 1000000))
}


#[cfg(test)]
mod tests {
    extern crate regex;

    use self::regex::Regex;
    use super::*;

    #[test]
    fn digits() {
        let re = Regex::new(r"^\d{6}(?:\s*,\s*\d{6})*$").unwrap();
        assert!(re.is_match(&hotp()));
    }

    #[test]
    fn len_6() {
        assert_eq!(hotp().len(), 6);
    }
}
