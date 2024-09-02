use crate::field_provider_v1::FieldElement;

/// Représentation de polynôme (une indéterminée - représentation de polynôme univarié)
#[derive(Clone, Debug)]
pub struct Polynome<T> {
    pub coefficients: Vec<T>,
}

fn remove_zeroes(coeffs: &[FieldElement]) -> Vec<FieldElement> {
    let mut no_zeroes_coefficients = coeffs
        .iter()
        .rev()
        .skip_while(|x| **x == FieldElement::from(0u64))
        .cloned()
        .collect::<Vec<FieldElement>>();
    no_zeroes_coefficients.reverse();
    no_zeroes_coefficients
}

// Pad with zero coefficients to length n
// Truncate if new length is less than size
fn pad_with_zero_coefficients_to_length(pa: &mut Polynome<FieldElement>, n: usize) {
    pa.coefficients.resize(n, FieldElement::from(0u64));
}

impl Polynome<FieldElement> {
    // Constructeur avec coefficients
    pub fn new_poly(coefficients: &[FieldElement]) -> Self {
        Polynome {
            coefficients: remove_zeroes(coefficients),
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
    
    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::from(0u64);
        let mut power = FieldElement::from(1u64);
        for coefficient in &self.coefficients {
            result += power * coefficient;
            power *= x;
        }
        result
    }

    pub fn evaluate_sliding(&self, input: &[FieldElement]) -> Vec<FieldElement> {
        input.iter().map(|x| self.evaluate(x)).collect()
    }

    /// Pads polynomial representations with minimum number of zeros to match lengths.
    pub fn pad_with_zero_coefficients(
        pa: &Polynome<FieldElement>,
        pb: &Polynome<FieldElement>,
    ) -> (Polynome<FieldElement>, Polynome<FieldElement>) {
        let mut pa = pa.clone();
        let mut pb = pb.clone();

        if pa.coefficients.len() > pb.coefficients.len() {
            pad_with_zero_coefficients_to_length(&mut pb, pa.coefficients.len());
        } else {
            pad_with_zero_coefficients_to_length(&mut pa, pb.coefficients.len());
        }
        (pa, pb)
    }

    pub fn fold_with_beta(&self, beta: &FieldElement) -> Polynome<FieldElement> {
        let coefs = self.coefficients.clone();

        let even_coefs = coefs
            .iter()
            .step_by(2)
            .cloned()
            .collect::<Vec<FieldElement>>();

        // Odd coefficients multiplied by beta (soc broken !!! - to refactor)
        let odd_coefs_betarized = coefs
            .iter()
            .skip(1)
            .step_by(2)
            .map(|x| x.clone() * beta)
            .collect::<Vec<FieldElement>>();

        let (even_poly, odd_poly) = Polynome::pad_with_zero_coefficients(
            &Polynome::new_poly(&even_coefs),
            &Polynome::new_poly(&odd_coefs_betarized),
        );

        let mut new_coefs = vec![];
        for (i, coef) in even_poly.coefficients.iter().enumerate() {
            new_coefs.push(coef.clone());
            if i < odd_poly.coefficients.len() {
                new_coefs[i] += odd_poly.coefficients[i].clone();
            }
        }

        return Polynome::new_poly(&new_coefs);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluate() {
        let coefficients = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
        ];
        let p = Polynome::new_poly(&coefficients);
        let x = FieldElement::from(2u64);
        let result = p.evaluate(&x);
        assert_eq!(result, FieldElement::from(17u64));
    }

    #[test]
    fn test_remove_zeroes() {
        let coefficients = vec![
            FieldElement::from(1u64),
            FieldElement::from(0u64),
            FieldElement::from(2u64),
            FieldElement::from(0u64),
            FieldElement::from(3u64),
            FieldElement::from(0u64),
        ];
        let result = remove_zeroes(&coefficients);
        assert_ne!(
            result,
            vec![
                FieldElement::from(1u64),
                FieldElement::from(2u64),
                FieldElement::from(3u64)
            ]
        );
        assert_eq!(
            result,
            vec![
                FieldElement::from(1u64),
                FieldElement::from(0u64),
                FieldElement::from(2u64),
                FieldElement::from(0u64),
                FieldElement::from(3u64)
            ]
        );
    }

    #[test]
    fn test_pad_with_zero_coefficients() {
        let coefficients1 = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
        ];
        let coefficients2 = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
            FieldElement::from(4u64),
            FieldElement::from(5u64),
        ];
        let p1 = Polynome::new_poly(&coefficients1);
        let p2 = Polynome::new_poly(&coefficients2);
        let (p1, p2) = Polynome::pad_with_zero_coefficients(&p1, &p2);

        assert_eq!(p1.coefficients.len(), 5);
        assert_eq!(p2.coefficients.len(), 5);
        assert_eq!(p1.coefficients[3], FieldElement::from(0u64));
    }

    #[test]
    fn test_evaluate_sliding() {
        let coefficients = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
        ];
        let p = Polynome::new_poly(&coefficients);
        let input0 = vec![FieldElement::from(0u64)];
        let input = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
        ];
        let result0 = p.evaluate_sliding(&input0);
        let result = p.evaluate_sliding(&input);
        assert_eq!(result0, vec![FieldElement::from(1u64)]);
        assert_eq!(
            result,
            vec![
                FieldElement::from(6u64),
                FieldElement::from(17u64),
                FieldElement::from(34u64)
            ]
        );
    }

    #[test]
    fn test_fold_with_beta() {
        let coefficients = vec![
            FieldElement::from(1u64),
            FieldElement::from(2u64),
            FieldElement::from(3u64),
            FieldElement::from(4u64),
            FieldElement::from(5u64),
            FieldElement::from(6u64),
        ];
        let p = Polynome::new_poly(&coefficients);
        let beta = FieldElement::from(2u64);
        let result = p.fold_with_beta(&beta);
        assert_eq!(result.degree(), p.degree() / 2); //Ugly but works
        assert_eq!(
            result.coefficients,
            vec![
                FieldElement::from(5u64),
                FieldElement::from(11u64),
                FieldElement::from(17u64),
            ]
        );
    }
}
