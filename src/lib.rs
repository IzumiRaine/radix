pub fn radix_sort_u32(input: &mut [u32])
{
    let mut output = vec![0u32; input.len()];
    let mut counters = [0u32; 256];  // radix 256
    radix_sort_u32_rec(input, &mut output, &mut counters, 0)
}

// Precondition: counters is filled with 0
fn radix_sort_u32_rec(input: &mut [u32], output: &mut [u32], counters: &mut [u32], round: u32)
{
    debug_assert!(input.len() == output.len());
    debug_assert!(counters.len() == 256);

    if round >= 4 {
        return;
    }

    let shift = 8 * round;

    let get_byte = |x: u32| {
        (x >> shift) & 0xFF
    };

    // Do counting sort on the round-th byte
    for &x in input.iter() {
        //let byte = (x >> round) & 0xFF;
        let byte = get_byte(x);
        counters[byte as usize] += 1;
    }

    // Compute prefix sum of counters
    let mut sum = 0;
    for count in counters.iter_mut() {
        sum += *count;
        *count = sum;
    }

    // Write to output array according to prefix sum
    for &x in input.iter().rev() {
        let byte = get_byte(x);
        counters[byte as usize] -= 1;
        let idx = counters[byte as usize];
        output[idx as usize] = x;
    }

    // Reset counters and proceed to next round
    counters.fill(0);
    radix_sort_u32_rec(output, input, counters, round + 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = [5, 3, 8, 7];
        radix_sort_u32(&mut data);
        assert_eq!(data, [3, 5, 7, 8]);
    }
}
