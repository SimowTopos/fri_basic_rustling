// Subject: FRI code layer implementation

use ff::{Field, PrimeField};
use rs_merkle::algorithms::Sha256;
use rs_merkle::Hasher;
use rs_merkle::MerkleTree;

use crate::channel::Channel;
use crate::field_provider_v1::FieldElement;
use crate::polynome::Polynome;

// Domain_size 8 time polynome degree
pub fn generate_enlarged_evaluation_domain(domain_size: usize) -> Vec<FieldElement> {
    let g = FieldElement::MULTIPLICATIVE_GENERATOR;
    let coset_offset = g.pow(&[(2u64.pow(30) * 3) % domain_size as u64]); // coset_offset outside the generator powers

    let coset = (0..domain_size)
        .map(|i| coset_offset.pow(&[i as u64]))
        .collect::<Vec<FieldElement>>(); //generated by the coset_offset

    return coset.iter().map(|x| g * x).collect::<Vec<FieldElement>>(); //acting on the coset to have the eval_domain
}

// Evaluate the polynomial on the enlarged domain
// In fact we need more sofisticated evaluation techniques to consider the  polynomial quotient
// As the FRI entry point couls be the quotient polynomial
// In this basic educational implementation we consider that this method can evaluate the polynomial quotient
// By segregating the numerator and the denominator and considering the product if any
pub fn evaluate_on_enlarged_domain(
    poly: &Polynome<FieldElement>,
    dom: &Vec<FieldElement>,
) -> Vec<FieldElement> {
    return poly.evaluate_sliding(dom);
}

pub fn build_next_domain(domain: &Vec<FieldElement>) -> Vec<FieldElement> {
    let fri_domain = domain.clone();
    let stop_index = fri_domain.len() / 2;
    let next_domain = fri_domain
        .iter()
        .map(|x| x.pow(&[2u64]))
        .take(stop_index)
        .collect::<Vec<FieldElement>>();
    return next_domain;
}

fn build_merkle_tree(values: &Vec<FieldElement>) -> MerkleTree<Sha256> {
    let mut leaves: Vec<[u8; 32]> = values
        .iter()
        .map(|x| {
            Sha256::hash(
                x.to_repr()
                    .as_ref()
                    .try_into()
                    .expect("Représentation incorrecte"),
            )
        })
        .collect();

    let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::new();

    merkle_tree.append(&mut leaves);
    merkle_tree.commit();

    // Return committed tree
    return merkle_tree;
}

#[derive(Clone)]
pub struct FriCodeLayer {
    pub evaluation: Vec<FieldElement>,
    pub domain: Vec<FieldElement>,
    pub merkle_tree: MerkleTree<Sha256>,
}

#[derive(Clone)]
pub struct FriDecommitment {
    pub layers_evaluations: Vec<FieldElement>,
    pub layers_auth_paths: Vec<Vec<[u8; 32]>>,
    pub layers_evaluations_sym: Vec<FieldElement>,
    pub layers_auth_paths_sym: Vec<Vec<[u8; 32]>>,
}

impl FriCodeLayer {
    pub fn new(poly: &Polynome<FieldElement>, dom: &Vec<FieldElement>) -> Self {
        let eval = evaluate_on_enlarged_domain(poly, dom);
        let mtree = build_merkle_tree(&eval);

        Self {
            evaluation: eval,
            domain: dom.to_vec(),
            merkle_tree: mtree,
        }
    }

    pub fn get_merkle_root(&self) -> Option<String> {
        return self.merkle_tree.root_hex();
    }

    // Commitment phase
    pub fn fri_commit_phase(
        initial_poly: Polynome<FieldElement>,
        domain_size: usize,
        interactive_channel: &mut Channel,
    ) -> (Polynome<FieldElement>, Vec<FriCodeLayer>) {
        //FieldElement
        let domain_size = domain_size;

        let mut fri_layer_list = Vec::with_capacity((initial_poly.degree() / 2) + 1);

        let initial_domain = generate_enlarged_evaluation_domain(domain_size);
        println!(
            "Initial domain generated with size : {:?}",
            initial_domain.len()
        );

        let mut current_layer = FriCodeLayer::new(&initial_poly, &initial_domain);
        println!("Initial layer generated");

        fri_layer_list.push(current_layer.clone());

        let mut current_poly = initial_poly;
        let mut current_domain = initial_domain;

        // >>>> Send commitment root
        // For the initial polynome we consider to map the merckle root to the 0 field element
        interactive_channel.add_committed_data(
            FieldElement::from(0u64),
            current_layer.merkle_tree.root_hex(),
        );

        while current_poly.degree() > 0 {
            println!(
                "Generate new layer with polynome degree : {:?}",
                current_poly.degree()
            );
            // <<<< Receive challenge
            let beta_challenge = interactive_channel.get_challenge();

            // Compute layer polynomial and domain
            // Next poly
            let next_poly = current_poly.fold_with_beta(&beta_challenge);
            // Next domain
            let next_domain = build_next_domain(&current_domain);

            // Compute next layer
            current_layer = FriCodeLayer::new(&next_poly, &next_domain);

            // >>>> Send commitment root
            interactive_channel.add_committed_data(beta_challenge, current_layer.get_merkle_root());
            println!(
                "Commitment root: {}",
                current_layer.get_merkle_root().unwrap_or_default()
            );

            fri_layer_list.push(current_layer);

            // Update current values

            current_poly = next_poly;
            current_domain = next_domain;
        }

        let last_poly = current_poly;

        return (last_poly, fri_layer_list);
    }

    // Decommitment phase
    pub fn fri_decommitment_phase(
        fri_number_of_queries: i32,
        domain_size: usize,
        fri_layers: &Vec<FriCodeLayer>,
        i_channel: &mut Channel,
    ) -> (Vec<FriDecommitment>, Vec<usize>) {
        if !fri_layers.is_empty() {
            let coef_index_queries = (0..fri_number_of_queries)
                .map(|_| (i_channel.get_index()) % domain_size)
                .collect::<Vec<usize>>();

            let query_list = coef_index_queries
                .iter()
                .map(|i| {
                    // <<<< Receive challenge index
                    let mut layers_evaluations = vec![];
                    let mut layers_auth_paths = vec![];
                    let mut layers_evaluations_sym = vec![];
                    let mut layers_auth_paths_sym = vec![];

                    for layer in fri_layers {
                        // symmetric element
                        let dom_size = layer.domain.len();

                        let index = i % dom_size;
                        let index_sym = (i + dom_size / 2) % dom_size;

                        let evaluation = layer.evaluation[index].clone();
                        let auth_path = layer.merkle_tree.proof(&[index]);
                        let auth_path_hashes = auth_path.proof_hashes();

                        let evaluation_sym = layer.evaluation[index_sym].clone();
                        let auth_path_sym = layer.merkle_tree.proof(&[index_sym]);
                        let auth_path_hashes_sym = auth_path_sym.proof_hashes();

                        layers_evaluations.push(evaluation);
                        layers_auth_paths.push(auth_path_hashes.to_vec());
                        layers_evaluations_sym.push(evaluation_sym);
                        layers_auth_paths_sym.push(auth_path_hashes_sym.to_vec());
                    }

                    FriDecommitment {
                        layers_evaluations,
                        layers_auth_paths,
                        layers_evaluations_sym,
                        layers_auth_paths_sym,
                    }
                })
                .collect();

            (query_list, coef_index_queries)
        } else {
            (vec![], vec![])
        }
    }
}

#[cfg(test)]
mod tests {

    use rs_merkle::MerkleProof;

    use super::*;

    #[test]
    fn test_generate_enlarged_evaluation_domain() {
        let domain_size = 5;
        let result = generate_enlarged_evaluation_domain(domain_size);
        assert_eq!(
            result,
            vec![
                FieldElement::from(7u64),
                FieldElement::from(343u64),
                FieldElement::from(16807u64),
                FieldElement::from(823543u64),
                FieldElement::from(40353607u64),
            ]
        );
    }

    #[test]
    fn test_evaluate_on_enlarged_domain() {
        let coefficients = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
        ];
        let p = Polynome::new_poly(&coefficients);
        let domain_size = 5;
        let dom = generate_enlarged_evaluation_domain(domain_size);
        let eval = evaluate_on_enlarged_domain(&p, &dom);
        assert_eq!(
            eval,
            vec![
                FieldElement::from(162u64),
                FieldElement::from(353634u64),
                FieldElement::from(847459362u64),
                FieldElement::from(2034670865634u64),
                FieldElement::from(4885240874438562u64),
            ]
        );
        assert_eq!(
            dom,
            vec![
                FieldElement::from(7u64),
                FieldElement::from(343u64),
                FieldElement::from(16807u64),
                FieldElement::from(823543u64),
                FieldElement::from(40353607u64),
            ]
        );
    }

    #[test]
    fn test_build_merkle_tree() {
        let values = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
            FieldElement::from(4u64),
            FieldElement::from(5u64),
            FieldElement::from(6u64),
        ];

        let merkle_tree = build_merkle_tree(&values);

        assert_eq!(
            merkle_tree.root_hex(),
            Some("864d91e7f731f52b93f048dc44142d8b4571500b7a01c3ea61f88f74f8c146df".to_string())
        );
    }

    #[test]
    #[should_panic(expected = "Symmetry should be respected")]
    fn test_eval_domain_symetry() {
        let domain_size = 10000;
        let domain = generate_enlarged_evaluation_domain(domain_size);
        let half_domain_size = domain.len() / 2; // Auto flooring

        assert_eq!(
            domain[100].pow(&[2u64]),
            domain[half_domain_size + 100].pow(&[2u64])
        ); //Issue on the domain generation to investigate
    }

    #[test]
    fn test_build_next_domain() {
        let domain = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
            FieldElement::from(4u64),
            FieldElement::from(5u64),
            FieldElement::from(6u64),
            FieldElement::from(7u64),
            FieldElement::from(8u64),
        ];

        let result = build_next_domain(&domain);

        assert_eq!(
            result,
            vec![
                FieldElement::from(1u64),
                FieldElement::from(4u64),
                FieldElement::from(9u64),
                FieldElement::from(16u64),
            ]
        );
    }

    #[test]
    fn test_fri_commit_phase() {
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
        let (last_poly, fri_layers) = FriCodeLayer::fri_commit_phase(poly, domain_size, i_channel);

        assert_eq!(fri_layers.len(), 4);
        assert_eq!(last_poly.degree(), 0);
    }

    #[test]
    fn test_fri_decommitment_phase() {
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
        let (last_poly, fri_layers) = FriCodeLayer::fri_commit_phase(poly, domain_size, i_channel);

        let (decom, queries) =
            FriCodeLayer::fri_decommitment_phase(3, domain_size, &fri_layers, i_channel);

        assert_eq!(last_poly.degree(), 0);
        assert_eq!(decom.len(), 3);
        assert_eq!(queries.len(), 3);
        decom.iter().for_each(|d| {
            assert_eq!(d.layers_evaluations.len(), 4);
            assert_eq!(d.layers_auth_paths.len(), 4);
            assert_eq!(d.layers_evaluations_sym.len(), 4);
            assert_eq!(d.layers_auth_paths_sym.len(), 4);

            (0..4).for_each(|i| {
                let proof_hashes = d.layers_auth_paths[i].clone();
                let proof = MerkleProof::<Sha256>::new(proof_hashes);

                let eval_hash = Sha256::hash(
                    d.layers_evaluations[i]
                        .to_repr()
                        .as_ref()
                        .try_into()
                        .expect("Représentation incorrecte"),
                );
                assert_eq!(hex::encode(eval_hash), proof.proof_hashes_hex()[0]);
            });
        });
    }
}
