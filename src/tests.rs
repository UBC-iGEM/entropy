use crate::*;

#[test]
fn test_simulate() {
    let mut buffer = RingBuffer::<f64>::with_capacity(5);
    for i in 0..100 {
        let (slice, slice_size) = buffer.slice_end(5);
        let v = simulate(0.0, 100.0, slice, slice_size, SimulationConfig::default());
        buffer.push(v);
        println!("{i} |  {v}");
    }
    println!("End: {:?}", buffer.slice_end(5));
}
