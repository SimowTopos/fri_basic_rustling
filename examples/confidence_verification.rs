use fri_basic_rustling::{
    channel::Channel,
    field_provider_v1::FieldElement,
    fri_code_layer::{FriCodeLayer, FriConfig},
    polynome::Polynome,
};

/// Example demonstrating confidence-based FRI verification
/// 
/// This example shows how to use the new FriConfig to specify 
/// a desired confidence level for the verification protocol.
fn main() {
    println!("FRI Confidence-Based Verification Example");
    println!("==========================================\n");

    // Create a test polynomial
    let coefficients = vec![
        FieldElement::from(1u64),
        FieldElement::from(2u64),
        FieldElement::from(3u64),
        FieldElement::from(4u64),
    ];
    let poly = Polynome::new_poly(&coefficients);
    let domain_size = 32; // Should be larger than polynomial degree
    
    println!("Polynomial degree: {}", poly.degree());
    println!("Domain size: {}\n", domain_size);

    // Run FRI commitment phase
    let mut channel = Channel::new();
    let (_last_poly, fri_layers) = FriCodeLayer::fri_commit_phase(poly, domain_size, &mut channel);
    
    println!("Commitment phase completed with {} layers\n", fri_layers.len());

    // Demonstrate different confidence levels
    let confidence_levels = [0.90, 0.95, 0.99, 0.999];
    
    for &confidence in &confidence_levels {
        println!("--- Testing {:.1}% Confidence Level ---", confidence * 100.0);
        
        match FriConfig::new(confidence) {
            Ok(config) => {
                // Calculate required queries
                let num_queries = config.calculate_num_queries();
                let actual_confidence = FriConfig::actual_confidence_level(num_queries);
                
                println!("Target confidence: {:.2}%", confidence * 100.0);
                println!("Queries required: {}", num_queries);
                println!("Actual confidence: {:.4}%", actual_confidence * 100.0);
                println!("Soundness error: {:.6}", 1.0 - actual_confidence);
                
                // Run verification with this confidence level
                let mut test_channel = Channel::new();
                let (_test_last_poly, test_fri_layers) = FriCodeLayer::fri_commit_phase(
                    Polynome::new_poly(&coefficients), 
                    domain_size, 
                    &mut test_channel
                );
                
                let (_decommitments, _queries, final_confidence) =
                    FriCodeLayer::fri_decommitment_phase_with_confidence(
                        &config,
                        domain_size,
                        &test_fri_layers,
                        &mut test_channel,
                    );
                
                println!("Verification result: {:.4}% confidence achieved\n", final_confidence * 100.0);
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
    }

    println!("Example completed!");
}