// lib.rs
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use zokrates_core::{compile::compile, ir::Prog, proof_system::*};
use zokrates_field::Bn128Field;

pub struct Voter {
    id: String,
    secret: u64,
}

pub struct VotingSystem {
    voters: HashMap<String, Voter>,
    votes: HashMap<String, bool>,
    compiled_program: Prog<Bn128Field>,
    proving_key: ProvingKey<Bn128Field>,
    verification_key: VerificationKey<Bn128Field>,
}

impl VotingSystem {
    pub fn new() -> Self {
        let source = r#"
            def main(private field secret, public field hash) -> bool:
                return sha256packed([secret]) == hash
        "#;
        let compiled = compile(source).unwrap();
        let (proving_key, verification_key) = setup(&compiled).unwrap();

        VotingSystem {
            voters: HashMap::new(),
            votes: HashMap::new(),
            compiled_program: compiled,
            proving_key,
            verification_key,
        }
    }

    pub fn register_voter(&mut self, id: &str) {
        let secret: u64 = rand::thread_rng().gen();
        let voter = Voter {
            id: id.to_string(),
            secret,
        };
        self.voters.insert(id.to_string(), voter);
    }

    pub fn get_voter_hash(&self, id: &str) -> Option<String> {
        self.voters.get(id).map(|voter| {
            let mut hasher = Sha256::new();
            hasher.update(&voter.secret.to_le_bytes());
            hex::encode(hasher.finalize())
        })
    }

    pub fn cast_vote(&mut self, id: &str, vote: bool, proof: &str) -> bool {
        if let Some(voter) = self.voters.get(id) {
            if self.verify_proof(voter.secret, proof) {
                self.votes.insert(id.to_string(), vote);
                return true;
            }
        }
        false
    }

    fn verify_proof(&self, secret: u64, proof: &str) -> bool {
        let proof: Proof<Bn128Field> = serde_json::from_str(proof).unwrap();
        let hash = self.compute_hash(secret);
        let inputs = vec![Bn128Field::from(hash)];
        verify(&self.verification_key, &proof, &inputs).unwrap()
    }

    fn compute_hash(&self, secret: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(&secret.to_le_bytes());
        let result = hasher.finalize();
        u64::from_le_bytes(result[..8].try_into().unwrap())
    }

    pub fn count_votes(&self) -> (usize, usize) {
        let yes_votes = self.votes.values().filter(|&&v| v).count();
        let no_votes = self.votes.values().filter(|&&v| !v).count();
        (yes_votes, no_votes)
    }

    pub fn generate_proof(&self, id: &str) -> Option<String> {
        if let Some(voter) = self.voters.get(id) {
            let hash = self.compute_hash(voter.secret);
            let witness = vec![Bn128Field::from(voter.secret), Bn128Field::from(hash)];
            let proof = prove(&self.compiled_program, &self.proving_key, witness).unwrap();
            Some(serde_json::to_string(&proof).unwrap())
        } else {
            None
        }
    }
}
