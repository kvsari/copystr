//! # Copy String
//! Strings that exist on the stack. This makes them `Copy`. Useful for when you want to
//! keep some small text inside a struct or enum and retain copy semantics. Strings are
//! stored as a byte array with UTF8 conversion on the fly.
use std::fmt;

const ERROR_MSG: &'static str = "&str len greater than max_len";

#[macro_export]
macro_rules! csstruct {
    ($css:ident, $asize:expr) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $css {            
            raw: [u8; $asize],            
            len: usize,
        }

        impl $css {
            pub fn max_len() -> usize {
                $asize
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn as_str(&self) -> &str {
                let (s, _) = self.raw.split_at(self.len);
                std::str::from_utf8(s).expect("Valid UTF8 invariant violated.")
            }

            pub fn as_bytes(&self) -> &[u8] {
                let (b, _) = self.raw.split_at(self.len);
                b
            }

            pub fn as_all_bytes(&self) -> &[u8] {
                &self.raw
            }
        }

        impl std::convert::TryFrom<&str> for $css {
            type Error = &'static str;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                if value.len() > $asize {
                    return Err(ERROR_MSG);
                }

                let mut raw: [u8; $asize] = [0; $asize];
                value
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, b)| raw[i] = *b);

                Ok($css {
                    raw,
                    len: value.len(),
                })
            }
        }

        impl fmt::Display for $css {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    };
}

csstruct!(s4, 4);
csstruct!(s8, 8);
csstruct!(s16, 16);
csstruct!(s32, 32);
    
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
        
    use super::*;
    
    #[test]
    fn copy_string_struct() {
        let cs = s4::try_from("ABC").unwrap();
        assert_eq!(cs.as_str(), "ABC");
    }

    #[test]
    fn display() {
        let cs = s4::try_from("XYZ").unwrap();
        assert_eq!(cs.to_string(), "XYZ");
    }
}
