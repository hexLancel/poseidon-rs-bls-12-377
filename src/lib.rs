extern crate rand;
#[macro_use]
extern crate ff;
use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "8444461749428370424248824938781546531375899335154063827935233455917409239041"]
#[PrimeFieldGenerator = "21"]
pub struct Fr(FrRepr);

mod constants;

#[derive(Debug)]
pub struct Constants {
    pub c: Vec<Vec<Fr>>,
    pub m: Vec<Vec<Vec<Fr>>>,
    pub n_rounds_f: usize,
    pub n_rounds_p: Vec<usize>,
}
pub fn load_constants() -> Constants {
    let (c_str, m_str) = constants::constants();
    let mut c: Vec<Vec<Fr>> = Vec::new();
    for i in 0..c_str.len() {
        let mut cci: Vec<Fr> = Vec::new();
        for j in 0..c_str[i].len() {
            let b: Fr = Fr::from_str(c_str[i][j]).unwrap();
            cci.push(b);
        }
        c.push(cci);
    }
    let mut m: Vec<Vec<Vec<Fr>>> = Vec::new();
    for i in 0..m_str.len() {
        let mut mi: Vec<Vec<Fr>> = Vec::new();
        for j in 0..m_str[i].len() {
            let mut mij: Vec<Fr> = Vec::new();
            for k in 0..m_str[i][j].len() {
                let b: Fr = Fr::from_str(m_str[i][j][k]).unwrap();
                mij.push(b);
            }
            mi.push(mij);
        }
        m.push(mi);
    }
    Constants {
        c: c,
        m: m,
        n_rounds_f: 8,
        n_rounds_p: vec![56, 57, 56, 60, 60, 63, 64, 63],
    }
}

pub struct Poseidon {
    constants: Constants,
}
impl Poseidon {
    pub fn new() -> Poseidon {
        Poseidon {
            constants: load_constants(),
        }
    }
    pub fn ark(&self, state: &mut Vec<Fr>, c: &Vec<Fr>, it: usize) {        
        for i in 0..state.len() {
            state[i].add_assign(&c[it + i]);
        }
    }

    pub fn sbox(&self, n_rounds_f: usize, n_rounds_p: usize, state: &mut Vec<Fr>, i: usize) {
        if i < n_rounds_f / 2 || i >= n_rounds_f / 2 + n_rounds_p {
            for j in 0..state.len() {
                let aux = state[j];
                state[j].square();
                state[j].square();
                state[j].mul_assign(&aux);
            }
        } else {
            let aux = state[0];
            state[0].square();
            state[0].square();
            state[0].mul_assign(&aux);
        }
    }

    pub fn mix(&self, state: &Vec<Fr>, m: &Vec<Vec<Fr>>) -> Vec<Fr> {
        let mut new_state: Vec<Fr> = Vec::new();
        for i in 0..state.len() {
            new_state.push(Fr::zero());
            for j in 0..state.len() {
                let mut mij = m[i][j];
                mij.mul_assign(&state[j]);
                new_state[i].add_assign(&mij);
            }
        }
        new_state.clone()
    }

    pub fn hash(&self, inp: Vec<Fr>) -> Result<Fr, String> {
        let t = inp.len() + 1;
        if inp.len() == 0 || inp.len() >= self.constants.n_rounds_p.len() - 1 {
            return Err("Wrong inputs length".to_string());
        }
        let n_rounds_f = self.constants.n_rounds_f.clone();
        let n_rounds_p = self.constants.n_rounds_p[t - 2].clone();

        let mut state = vec![Fr::zero(); t];
        state[1..].clone_from_slice(&inp);

        for i in 0..(n_rounds_f + n_rounds_p) {
            self.ark(&mut state, &self.constants.c[t - 2], i * t);
            self.sbox(n_rounds_f, n_rounds_p, &mut state, i);
            state = self.mix(&state, &self.constants.m[t - 2]);
        }

        Ok(state[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let b1: Fr = Fr::from_str("1").unwrap();
        let b2: Fr = Fr::from_str("2").unwrap();

        let mut big_arr: Vec<Fr> = Vec::new();
        big_arr.push(b1.clone());
        big_arr.push(b2.clone());
        let poseidon = Poseidon::new();
        let h = poseidon.hash(big_arr.clone()).unwrap();
        assert_eq!(
            h.to_string(),
            "Fr(0x0ee79c570dd490a23b9d19037e0b20c5390b1cae9a6fd1c421233ca41408d396)"
        );
    }
}
