// main.rs
use zkp_voting_system::VotingSystem;

fn main() {
    let mut voting_system = VotingSystem::new();

    // Register voters
    voting_system.register_voter("Alice");
    voting_system.register_voter("Bob");
    voting_system.register_voter("Charlie");

    // Generate and cast votes
    if let Some(proof) = voting_system.generate_proof("Alice") {
        assert!(voting_system.cast_vote("Alice", true, &proof));
    }

    if let Some(proof) = voting_system.generate_proof("Bob") {
        assert!(voting_system.cast_vote("Bob", false, &proof));
    }

    if let Some(proof) = voting_system.generate_proof("Charlie") {
        assert!(voting_system.cast_vote("Charlie", true, &proof));
    }

    // Count votes
    let (yes_votes, no_votes) = voting_system.count_votes();
    println!("Yes votes: {}, No votes: {}", yes_votes, no_votes);

    // Display voter hashes (in a real system, these would be public)
    for voter in ["Alice", "Bob", "Charlie"] {
        if let Some(hash) = voting_system.get_voter_hash(voter) {
            println!("{}'s hash: {}", voter, hash);
        }
    }
}
