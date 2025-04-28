/*

COMPARING PERFORMANCE => LOOPS VS ITERATORS



*/

iterators are one of rust's zero cost abstractions ,
which means using the abstractuon imposes no additional runtime overhead


in general, C++ imlpementations obey the zero overhead principle, what you dont use, myou dont pay for
what you do use, you couldn;t had code any better

//---------------
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
//----------------


closures and iterators are rust features innspired by functional programminf language ideas, 
they contribute to rusts capacibility to clearly express high level ideas at lw level perfoemnace, rhe
imlpementations of closures and iterators are such that the tuntime
performance is not affected 
this ispart of rusts goal to strive to provide zero cosr abstractions