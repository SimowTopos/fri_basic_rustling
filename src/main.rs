use channel::Channel;
use ff::PrimeField;
use field_provider_v1::FieldElement;
use fri_code_layer::{FriCodeLayer, FriConfig};
use polynome::Polynome;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof};

pub mod channel;
pub mod field_provider_v1;
pub mod fri_code_layer;
pub mod polynome;

fn main() {
    let coefficients = vec![
        FieldElement::from(1u64),
        FieldElement::from(2u64),
        FieldElement::from(3u64),
        FieldElement::from(3u64),
        FieldElement::from(3u64),
        FieldElement::from(3u64),
        FieldElement::from(3u64),
    ];
    let poly = Polynome::new_poly(&coefficients);
    let domain_size = 48; // 8 time degree of the polynome
    let i_channel = &mut Channel::new();

    println!("COMMITMENT PHASE");
    let (_last_poly, fri_layers) = FriCodeLayer::fri_commit_phase(poly, domain_size, i_channel);

    println!("\nDECOMMITMENT PHASE - Original method (20 queries)");
    let (decom_original, _queries_original) =
        FriCodeLayer::fri_decommitment_phase(20, domain_size, &fri_layers, i_channel);

    println!("Original method used 20 queries");

    // Reset channel for new decommitment
    *i_channel = Channel::new();
    let (_last_poly2, _fri_layers2) = FriCodeLayer::fri_commit_phase(
        Polynome::new_poly(&coefficients), 
        domain_size, 
        i_channel
    );

    println!("\nDECOMMITMENT PHASE - Confidence-based verification");
    
    // Demonstrate different confidence levels
    let confidence_levels = vec![0.90, 0.95, 0.99, 0.999];
    
    for confidence_level in confidence_levels {
        println!("\n--- Testing {:.1}% confidence level ---", confidence_level * 100.0);
        
        match FriConfig::new(confidence_level) {
            Ok(config) => {
                let num_queries = config.calculate_num_queries();
                let actual_confidence = FriConfig::actual_confidence_level(num_queries);
                
                println!("Target confidence: {:.2}%", confidence_level * 100.0);
                println!("Calculated queries needed: {}", num_queries);
                println!("Actual confidence achieved: {:.4}%", actual_confidence * 100.0);
                
                // Reset channel for this test
                *i_channel = Channel::new();
                let (_last_poly_test, fri_layers_test) = FriCodeLayer::fri_commit_phase(
                    Polynome::new_poly(&coefficients), 
                    domain_size, 
                    i_channel
                );
                
                let (_decom_confidence, _queries_confidence, achieved_confidence) =
                    FriCodeLayer::fri_decommitment_phase_with_confidence(
                        &config,
                        domain_size,
                        &fri_layers_test,
                        i_channel,
                    );
                
                println!("Verification completed with {:.4}% confidence", achieved_confidence * 100.0);
            }
            Err(e) => {
                println!("Error creating config: {}", e);
            }
        }
    }

    // Show verification of one set of decommitments
    println!("\n--- Verification Example ---");
    decom_original.iter().take(1).for_each(|d| {
        println!("MANAGE NEXT QUERY");
        (0..d.layers_evaluations.len()).for_each(|i| {
            println!("MANAGE NEXT LAYER");
            let proof_hashes = d.layers_auth_paths[i].clone();
            let proof = MerkleProof::<Sha256>::new(proof_hashes);

            
            let eval_hash = Sha256::hash(
                d.layers_evaluations[i]
                    .to_repr()
                    .as_ref()
                    .try_into()
                    .expect("Représentation incorrecte"),
            );

            println!("Proof path for layer {:?}: {:?}", i, proof.proof_hashes_hex());
            println!("Hash for layer {:?} : {:?}", i, hex::encode(eval_hash));
        });
    });
}
