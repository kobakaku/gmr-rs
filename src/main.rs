use gmw_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GMW Protocol Implementation - XOR and NOT gates only");

    let circuit_file = load_circuit_from_file("circuits/simple.json")?;
    println!("Loaded circuit file: {}", circuit_file.name);

    let circuit = get_circuit_by_id(&circuit_file, "XOR_NOT_example").ok_or("Circuit not found")?;

    println!("Running circuit: {}", circuit.id);

    // Test case 1: true XOR false = true, NOT(true) = false
    let alice_input = true;
    let bob_input = false;

    let (alice_share_input1, bob_share_input1) = secret_share(alice_input);
    let (alice_share_input2, bob_share_input2) = secret_share(bob_input);

    let mut alice = Party::new(0);
    let mut bob = Party::new(1);

    alice.set_share(circuit.alice[0], alice_share_input1);
    bob.set_share(circuit.alice[0], bob_share_input1);

    alice.set_share(circuit.bob[0], alice_share_input2);
    bob.set_share(circuit.bob[0], bob_share_input2);

    alice.evaluate_circuit(circuit);
    bob.evaluate_circuit(circuit);

    let output_wire = circuit.out[0];
    let alice_output = alice.get_share(output_wire).unwrap();
    let bob_output = bob.get_share(output_wire).unwrap();

    let final_result = reconstruct_shares(alice_output, bob_output);

    println!("Alice input: {alice_input}");
    println!("Bob input: {bob_input}");
    println!("Alice shares: input1={alice_share_input1}, input2={alice_share_input2}");
    println!("Bob shares: input1={bob_share_input1}, input2={bob_share_input2}");
    println!("Alice output share: {alice_output}");
    println!("Bob output share: {bob_output}");
    println!("Final result: {final_result}");

    let xor_result = alice_input ^ bob_input;
    let expected = !xor_result;
    println!("XOR result: {xor_result}");
    println!("Expected result: {expected} (NOT(XOR({alice_input}, {bob_input})))");
    println!("Result matches: {}", final_result == expected);

    Ok(())
}
