use crate::adjectives::ADJECTIVES;
use crate::nouns::NOUNS;
use anyhow::{Result, anyhow};
use rand::seq::IndexedRandom;

/// Generates a random (docker-like) name made up of 0+ adjectives
/// and one noun, joined by a separating character.
///
/// # Arguments:
///
/// * `nadj`: The number of adjectives (default is 1)
/// * `sep`: The separating character (default is '-')
///
pub fn rand_phrase(nadj: Option<u8>, sep: Option<char>) -> Result<String> {
    let mut rng = rand::rng();
    let mut words: Vec<String> = vec![];

    // Add zero or more adjectives
    for _ in 0..nadj.unwrap_or(1) {
        let w = match ADJECTIVES.choose(&mut rng) {
            Some(s) => s.to_string(),
            None => return Err(anyhow!("Failed to get an adjective")),
        };
        words.push(w);
    }

    // Add a final noun
    let w = match NOUNS.choose(&mut rng) {
        Some(s) => s.to_string(),
        None => return Err(anyhow!("Failed to get a noun")),
    };
    words.push(w);

    // Join with the separator and return
    let sep = sep.unwrap_or('-').to_string();
    let name = words.join(&sep);
    Ok(name)
}
