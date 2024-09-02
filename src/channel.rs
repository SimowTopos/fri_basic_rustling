use rand::Rng;
use std::collections::HashMap;

use crate::field_provider_v1::FieldElement;

#[derive(Clone, Debug)]
pub struct Channel {
    committed_merkle_root_by_challenge: HashMap<FieldElement, Option<String>>,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            committed_merkle_root_by_challenge: HashMap::new(),
        }
    }

    pub fn get_challenge(&self) -> FieldElement {
        return FieldElement::from(rand::thread_rng().gen::<u64>());
    }

    pub fn get_index(&self) -> usize {
        return rand::thread_rng().gen::<usize>();
    }

    pub fn add_committed_data(
        &mut self,
        beta_challenge: FieldElement,
        merkel_root: Option<String>,
    ) {
        self.committed_merkle_root_by_challenge
            .insert(beta_challenge, merkel_root);
    }

    pub fn get_merkle_root(&self, beta_challenge: FieldElement) -> Option<String> {
        self.committed_merkle_root_by_challenge
            .get(&beta_challenge)
            .cloned()
            .flatten()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_channel() {
        let mut channel = Channel::new();
        let beta_challenge = channel.get_challenge();
        let merkle_root = Some("0x1234".to_string());
        channel.add_committed_data(beta_challenge, merkle_root.clone());

        assert_eq!(channel.get_merkle_root(beta_challenge), merkle_root);
    }
}
