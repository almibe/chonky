// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use chonky::{Chonky, HandlerError, Messages};
    use serde::{Deserialize, Serialize};
    use std::iter;
    use std::any::Any;

    #[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    fn increase_age(input: Messages) -> Result<Messages, HandlerError> {
        Ok(to_messages(from_messages(input).map(|p| {
            Person {
                name: p.name,
                age: p.age + 1,
            }
        })))
    }

    fn reverse_name(input: Messages) -> Result<Messages, HandlerError> {
        Ok(to_messages(from_messages(input).map(|p| {
            Person {
                name: p.name.chars().rev().collect(),
                age: p.age,
            }
        })))
    }

    fn to_messages(people: impl Iterator<Item = Person> + 'static) -> Messages {
        let itr = people.map(|p| Box::new(p) as Box<dyn Any>);
        Box::new(itr)
    }

    fn decode_from_any(any: Box<dyn Any>) -> Person {
        match any.downcast_ref::<Person>() {
            Some(p) => {
                p.clone()
            }
            None => {
                todo!()
            }
        }
    }
    
    fn from_messages(messages: Messages) -> impl Iterator<Item = Person> {
        messages.map(|m| decode_from_any(m))
    }

    #[test]
    fn dead_letter_check() {
        let c = Chonky::new();
        let p = Person {
            name: String::from("Emile"),
            age: 55,
        };
        let res = c.post(String::from("Hello"), to_messages(iter::once(p)));
        assert!(res.is_err()); //TODO should check it's a DeadLetter not just an Err
    }

    #[test]
    fn add_an_addressee_and_send_message() {
        let mut c = Chonky::new();
        c.register_addressee(String::from("increase_age"), increase_age);
        let person_itr = iter::once(Person {
            name: String::from("Emile"),
            age: 55,
        });
        let res_messages = c.post(String::from("increase_age"), to_messages(person_itr)).unwrap();
        let res_person: Vec<Person> = from_messages(res_messages).collect();
        let expected = vec![Person {
            name: String::from("Emile"),
            age: 56,
        }];
        assert_eq!(res_person, expected);
    }

    #[test]
    fn add_two_addressees_and_send_messages() {
        let mut c = Chonky::new();
        c.register_addressee(String::from("increase_age"), increase_age);
        c.register_addressee(String::from("reverse_name"), reverse_name);
        let person_itr = iter::once(Person {
            name: String::from("Emile"),
            age: 55,
        });
        let res1 = c.post(String::from("increase_age"), to_messages(person_itr)).unwrap();
        let res2 = c.post(String::from("reverse_name"), res1).unwrap();
        let res_person: Vec<Person> = from_messages(res2).collect();
        let expected = vec![Person {
            name: String::from("elimE"),
            age: 56,
        }];
        assert_eq!(res_person, expected);
    }
}
