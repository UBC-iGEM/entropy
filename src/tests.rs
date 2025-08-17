use crate::*;

#[test]
fn test_simulate() {
        let mut buffer = RingBuffer::<f64>::with_capacity(5);
        for i in 0..100 {
                let v = simulate(0.0, 100.0, &mut buffer, SimulationConfig::default());
                println!("{i} |  {v}");
        }
}
