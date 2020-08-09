use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;

pub mod functions;
pub mod types;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Identifier(String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub struct IllegalCharacterInIdentifier(char);

impl Debug for IllegalCharacterInIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal character in identifier: {}", self.0)
    }
}

impl Display for IllegalCharacterInIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IllegalCharacterInIdentifier {}

impl Identifier {
    pub fn new(name: String) -> Result<Self, IllegalCharacterInIdentifier> {
        for (index, c) in name.char_indices() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {}
                '0'..='9' if index > 0 => {}
                illegal => return Err(IllegalCharacterInIdentifier(illegal)),
            }
        }

        Ok(Identifier(name))
    }
}

impl AsRef<String> for Identifier {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum FullIdentifier {
    Name(Identifier),
    Namespaced(Identifier, Box<FullIdentifier>),
}

impl FullIdentifier {
    pub fn get_name(&self) -> &Identifier {
        match self {
            FullIdentifier::Name(name) => name,
            FullIdentifier::Namespaced(_, lower) => lower.get_name(),
        }
    }

    pub fn is_sub_identifier_of(&self, other: &Self) -> bool {
        match (self, other) {
            (FullIdentifier::Name(name), FullIdentifier::Name(other_name)) => name == other_name,
            (FullIdentifier::Namespaced(..), FullIdentifier::Name(_)) => false,
            (FullIdentifier::Name(name), FullIdentifier::Namespaced(current, _)) => name == current,
            (
                FullIdentifier::Namespaced(name, self_next),
                FullIdentifier::Namespaced(other_name, other_next),
            ) => {
                if name == other_name {
                    self_next.is_sub_identifier_of(other_next)
                } else {
                    false
                }
            }
        }
    }

    pub fn other_is_sub_identifier(&self, other: &Self) -> bool {
        other.is_sub_identifier_of(self)
    }

    pub fn remove(self, other: &Self) -> Result<Option<Self>, ()> {
        if !other.is_sub_identifier_of(&self) {
            return Err(());
        }

        if &self == other {
            return Ok(None);
        }

        let mut self_iter = IntoIterator::into_iter(self);
        let mut other_iter = IntoIterator::into_iter(other);

        while let Some(other) =  other_iter.next()  {
            if let Some(this) = self_iter.next() {
                if & this != other {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Some(FromIterator::from_iter(self_iter)))
    }
}

impl FromIterator<Identifier> for FullIdentifier {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        let collected: Vec<_> = iter.into_iter().collect();
        let mut reversed = collected.into_iter().rev();
        let mut output = FullIdentifier::Name(reversed.next().unwrap());
        for identifier in reversed {
            output = FullIdentifier::Namespaced(identifier, Box::new(output));
        }
        output
    }
}

impl<S: AsRef<str>> FromIterator<S> for FullIdentifier {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let id_iter = iter.into_iter().map(|s: S| Identifier::from(s.as_ref()));
        FullIdentifier::from_iter(id_iter)
    }
}

#[macro_export]
macro_rules! identifier {
    ($($identifier:ident)::+) => {
        $crate::resolution::FullIdentifier::from_iter(vec![$(stringify!($identifier)),*])
    };

}

pub struct IdentifierIter(Option<FullIdentifier>);

impl Iterator for IdentifierIter {
    type Item = Identifier;

    fn next(&mut self) -> Option<Self::Item> {
        let current = std::mem::replace(&mut self.0, None);
        let (out, next): (Option<Identifier>, Option<FullIdentifier>) = match current {
            None => (None, None),
            Some(FullIdentifier::Name(last)) => (Some(last), None),
            Some(FullIdentifier::Namespaced(out, next)) => (Some(out), Some(*next)),
        };
        std::mem::replace(&mut self.0, next);
        out
    }
}

impl IntoIterator for FullIdentifier {
    type Item = Identifier;
    type IntoIter = IdentifierIter;

    fn into_iter(self) -> Self::IntoIter {
        IdentifierIter(Some(self))
    }
}

impl<'a> IntoIterator for &'a FullIdentifier {
    type Item = &'a Identifier;
    type IntoIter = std::vec::IntoIter<&'a Identifier>;

    fn into_iter(self) -> Self::IntoIter {
        let mut output = vec![];
        let mut current = self;
        loop {
            match current {
                FullIdentifier::Name(last) => {
                    output.push(last);
                    break;
                }
                FullIdentifier::Namespaced(namespace, next) => {
                    output.push(namespace);
                    current = &*next;
                }
            }
        }

        output.into_iter()
    }
}

impl Display for FullIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<String> = self
            .into_iter()
            .map(|ident| ident.as_ref())
            .cloned()
            .collect::<Vec<_>>();
        write!(f, "{}", strings.join("::"))
    }
}

impl From<String> for Identifier {
    /// Converts from a string to an identifier
    ///
    /// # Panic
    ///
    /// Panics if `s` is an invalid identifier
    fn from(s: String) -> Identifier {
        Identifier::new(s).unwrap()
    }
}

impl<'a> From<&'a str> for Identifier {
    /// Converts from a `&str` to an identifier
    ///
    /// # Panic
    ///
    /// Panics if `s` is an invalid identifier
    fn from(s: &'a str) -> Identifier {
        Identifier::from(s.to_string())
    }
}

impl From<Identifier> for FullIdentifier {
    fn from(s: Identifier) -> Self {
        FullIdentifier::Name(s)
    }
}

impl<S: AsRef<str>> From<S> for FullIdentifier {
    fn from(s: S) -> Self {
        FullIdentifier::Name(Identifier::from(s.as_ref()))
    }
}

pub trait Resolvable {
    fn get_identifier(&self) -> &FullIdentifier;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn detect_valid_identifiers() {
        Identifier::from("x1");
        Identifier::from("_x");
        Identifier::from("_1");
        Identifier::from("lights_on");
    }

    #[test]
    #[should_panic]
    fn detect_numeral_first() {
        Identifier::from("1x");
    }

    #[test]
    #[should_panic]
    fn detect_spaces() {
        Identifier::from("1 x");
    }

    #[test]
    #[should_panic]
    fn detect_illegal_chars() {
        Identifier::from("x#");
    }

    #[test]
    fn full_identifiers() {
        let namespaced = FullIdentifier::from_iter(&["std", "Object"]);
        let namespaced_str = format!("{}", namespaced);
        assert_eq!(&*namespaced_str, "std::Object");

        let triple_namespaced =
            FullIdentifier::Namespaced(Identifier::from("modular"), Box::new(namespaced));
        let namespaced_str = format!("{}", triple_namespaced);
        assert_eq!(&*namespaced_str, "modular::std::Object");
    }

    #[test]
    fn identifier_equality() {
        let vec1= identifier!(std::Object);
        let vec2 = identifier!(std::Object);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn is_sub_identifier() {
        let long = identifier!(std::Object::hash_code);
        let short = identifier!(std::Object);
        assert!(short.is_sub_identifier_of(&long));
        assert!(long.other_is_sub_identifier(&short));
        assert!(!long.is_sub_identifier_of(&short));
    }

    #[test]
    fn get_pos_sub_identifier() {
        let long = identifier!(std::Object::hash_code);
        let short = identifier!(std::Object);

        assert_eq!(long.clone().remove(&short), Ok(Some(identifier!(hash_code))));
        assert_eq!(long.clone().remove(&long), Ok(None));
        assert_eq!(short.remove(&long), Err(()));

    }
}
