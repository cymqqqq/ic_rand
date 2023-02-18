use utils::Rng;

pub mod utils;
pub mod distribution;
pub mod core;
pub mod uniform;
// pub mod slice;
pub mod other;
pub mod bernouilli;

pub fn gen_f32() -> f32 {
    let seed = 12345u64;
    let mut rng = utils::Rand32::new(seed);
    rng.rand_f32()
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn gen() {
        let result = gen_f32();
        println!("random f32 {:?}", result);
    }
}
