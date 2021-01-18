// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use chonky::Chonky;

    #[test]
    fn dead_letter_check() {
        let c = Chonky::new();
        let res = c.post(String::from("Hello"), vec!());
        assert!(res.is_err());
    }
}
