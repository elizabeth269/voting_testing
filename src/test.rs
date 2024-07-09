// tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voter_registration() {
        let mut system = VotingSystem::new().unwrap();
        system.register_voter("Alice");
        assert!(system
            .get_registered_voters()
            .contains(&"Alice".to_string()));
    }

    #[test]
    fn test_voting_process() {
        let mut system = VotingSystem::new().unwrap();
        system.register_voter("Bob");

        let proof = system.generate_proof("Bob").unwrap();
        assert!(system.cast_vote("Bob", true, &proof).is_ok());

        let (yes_votes, no_votes) = system.count_votes();
        assert_eq!(yes_votes, 1);
        assert_eq!(no_votes, 0);
    }

    #[test]
    fn test_double_voting() {
        let mut system = VotingSystem::new().unwrap();
        system.register_voter("Charlie");

        let proof = system.generate_proof("Charlie").unwrap();
        assert!(system.cast_vote("Charlie", true, &proof).is_ok());
        assert!(system.cast_vote("Charlie", false, &proof).is_err());
    }

    #[test]
    fn test_unregistered_voter() {
        let system = VotingSystem::new().unwrap();
        assert!(system.generate_proof("Dave").is_err());
    }

    #[test]
    fn test_invalid_proof() {
        let mut system = VotingSystem::new().unwrap();
        system.register_voter("Eve");

        let invalid_proof = "invalid_proof_string";
        assert!(system.cast_vote("Eve", true, invalid_proof).is_err());
    }

    #[test]
    fn test_voter_hash_consistency() {
        let mut system = VotingSystem::new().unwrap();
        system.register_voter("Frank");

        let hash1 = system.get_voter_hash("Frank");
        let hash2 = system.get_voter_hash("Frank");

        assert_eq!(hash1, hash2);
    }
}
