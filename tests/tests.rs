// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use chonky::Chonky;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    fn increase_age(serialized_person: Vec<u8>) -> Vec<u8> {
        let mut decoded: Person = bincode::deserialize(&serialized_person[..]).unwrap();
        decoded.age = decoded.age + 1;
        bincode::serialize(&decoded).unwrap()
    }

    #[test]
    fn dead_letter_check() {
        let c = Chonky::new();
        let res = c.post(String::from("Hello"), vec![]);
        assert!(res.is_err()); //TODO should check it's a DeadLetter not just an Err
    }

    #[test]
    fn add_an_addressee_add_send_message() {
        let mut c = Chonky::new();
        c.register_addressee(String::from("increase_age"), increase_age);
        let person = Person {
            name: String::from("Emile"),
            age: 55,
        };
        let encoded: Vec<u8> = bincode::serialize(&person).unwrap();
        let res_encoded = c.post(String::from("increase_age"), encoded).unwrap();
        let res_person: Person = bincode::deserialize(&res_encoded[..]).unwrap();
        let expected = Person {
            name: String::from("Emile"),
            age: 56,
        };
        assert_eq!(res_person, expected);
    }
}
