use honggfuzz::fuzz;
use microbin::animalnumbers::to_u64;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(s) = std::str::from_utf8(data) {
                // Call the to_u64 function with the fuzzed input
                let _ = to_u64(s);
                

            }
        });
    }
}
