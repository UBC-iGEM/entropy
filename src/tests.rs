use crate::*;

#[test]
fn test_simulate() {
        let mut buffer = RingBuffer::<f64>::with_capacity(5);
        let config = SimulationConfig::default();
        for i in 0..100 {
                let v = simulate(0.0, 100.0, &mut buffer, &config);
                println!("{i} |  {v}");
        }
}
