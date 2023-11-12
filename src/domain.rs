use serde::{Deserialize, Serialize};
pub struct SubscriberName(String);
use unicode_segmentation::UnicodeSegmentation;
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailPost {
    pub title: String,
}
impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set, // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;
        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            // panic!("{} is not a valid subscriber name.", s)
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

pub trait AsRef<T: ?Sized> {
    /// Performs the conversion.
    fn as_ref(&self) -> &T;
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[test]
fn dummy_fail() {
    let result: Result<&str, &str> = Err("The app crashed due to an IO error");
    assert!(result.is_ok());
}
