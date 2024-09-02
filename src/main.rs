use channel::Channel;
use ff::PrimeField;
use field_provider_v1::FieldElement;
use fri_code_layer::FriCodeLayer;
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
    let (_last_poly, fri_layers) = FriCodeLayer::fri_commit_phase(poly, domain_size, i_channel);

    let (decom, _queries) =
        FriCodeLayer::fri_decommitment_phase(20, domain_size, &fri_layers, i_channel);

    decom.iter().for_each(|d| {
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
