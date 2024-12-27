use regex;

pub enum SolanaRegex {
    Pubkey,
    Keypair,
}

impl SolanaRegex {
    pub fn to_regex(self) -> regex::Regex {
        match self {
            Self::Pubkey => {
                regex::Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{32,44}$")
                    .unwrap()},
            Self::Keypair => regex::Regex::new(r"^[A-Za-z0-9]{88}$").unwrap(),
        }
    }
}

pub struct Account {}

impl Account {
    pub fn is_valid_public_key(public_key: String) -> bool {
        SolanaRegex::Pubkey.to_regex().is_match(public_key.as_str())
    }

    pub fn is_valid_private_key(private_key: String) -> bool {
        SolanaRegex::Keypair
            .to_regex()
            .is_match(private_key.as_str())
    }
}

fn main() {}


#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk;

    #[test]
    fn check_public_key() {
        let public_key = solana_sdk::pubkey::Pubkey::new_unique();
        assert!(Account::is_valid_public_key(public_key.to_string()))
    }

    #[test]
    fn check_private_key() {
        let private_key = solana_sdk::signer::keypair::Keypair::new();
        assert!(Account::is_valid_private_key(private_key.to_base58_string()))
    }
}
