//! Integration tests for the VendingMachine smart contract
//!
//! These tests verify the contract's behavior from an external perspective,
//! simulating real blockchain interactions using the Stylus SDK's TestVM.

use stylus_cupcake_example::VendingMachine;
use stylus_sdk::alloy_primitives::{address, U256};
use stylus_sdk::testing::*;

#[test]
fn test_give_cupcake_to() {
    let vm = TestVM::default();

    let mut contract = VendingMachine::from(&vm);
    let user = address!("0xCDC41bff86a62716f050622325CC17a317f99404");
    assert_eq!(contract.get_cupcake_balance_for(user).unwrap(), U256::ZERO);

    vm.set_block_timestamp(vm.block_timestamp() + 6);

    // Give a cupcake and verify it succeeds
    assert!(contract.give_cupcake_to(user).unwrap());

    // Check balance is now 1
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(1)
    );

    // Try to give another cupcake immediately - should fail due to time restriction
    assert!(!contract.give_cupcake_to(user).unwrap());

    // Balance should still be 1
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(1)
    );

    // Advance block timestamp by 6 seconds
    vm.set_block_timestamp(vm.block_timestamp() + 6);

    // Now giving a cupcake should succeed
    assert!(contract.give_cupcake_to(user).unwrap());

    // Balance should now be 2
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(2)
    );
}

/// This test demonstrates advanced configuration and usage of the TestVM for
/// comprehensive smart contract testing.
///
/// It covers:
/// - Creating and configuring a TestVM with custom parameters
/// - Setting blockchain state (timestamps, block numbers)
/// - Interacting with contract methods
/// - Taking and inspecting VM state snapshots
/// - Mocking external contract calls
/// - Testing time-dependent contract behavior
#[test]
fn test_advanced_testvm_configuration() {
    // SECTION 1: TestVM Setup and Configuration
    // -----------------------------------------

    // Create a TestVM with custom configuration using the builder pattern
    // This approach allows for fluent, readable test setup
    let vm: TestVM = TestVMBuilder::new()
        // Set the transaction sender address (msg.sender in Solidity)
        .sender(address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
        // Set the address where our contract is deployed
        .contract_address(address!("0x5FbDB2315678afecb367f032d93F642f64180aa3"))
        // Set the ETH value sent with the transaction (msg.value in Solidity)
        .value(U256::from(1))
        .build();

    // Configure additional blockchain state parameters directly on the VM instance
    // This demonstrates how to set parameters after VM creation
    vm.set_block_number(12345678);

    // Note: The chain ID is set to 42161 (Arbitrum One) by default in the TestVM
    // We don't need to set it explicitly as it's already configured in the VM state

    // SECTION 2: Contract Initialization and User Setup
    // ------------------------------------------------

    // Initialize our VendingMachine contract with the configured VM
    // The `from` method connects our contract to the test environment
    let mut contract = VendingMachine::from(&vm);

    // Define a user address that will interact with our contract
    // This represents an external user's Ethereum address
    let user = address!("0xCDC41bff86a62716f050622325CC17a317f99404");

    // SECTION 3: Initial State Verification
    // ------------------------------------

    // Verify the user starts with zero cupcakes
    // This confirms our contract's initial state is as expected
    assert_eq!(contract.get_cupcake_balance_for(user).unwrap(), U256::ZERO);

    // Set the initial block timestamp by advancing it by 10 seconds
    // This ensures we're past any time-based restrictions
    vm.set_block_timestamp(vm.block_timestamp() + 10);

    // SECTION 4: Contract Interaction
    // ------------------------------

    // Give a cupcake to the user and verify the operation succeeds
    // The contract should return true when a cupcake is successfully given
    assert!(contract.give_cupcake_to(user).unwrap());

    // Verify the user now has exactly one cupcake
    // This confirms our contract correctly updated its storage
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(1)
    );

    // SECTION 5: VM State Inspection
    // -----------------------------

    // Take a snapshot of the current VM state for inspection
    // This captures all storage, balances, and blockchain parameters
    let snapshot = vm.snapshot();

    // Inspect various aspects of the VM state to verify configuration
    // Chain ID should be Arbitrum One (42161) which is the default
    assert_eq!(snapshot.chain_id, 42161);
    // Message value should match what we configured (1 wei)
    assert_eq!(snapshot.msg_value, U256::from(1));

    // SECTION 6: Mocking External Contract Calls
    // -----------------------------------------

    // Define an external contract we might want to interact with
    let external_contract = address!("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199");
    // Define example call data we would send to that contract
    let call_data = vec![0xab, 0xcd, 0xef];
    // Define the expected response from that contract
    let expected_response = vec![0x12, 0x34, 0x56];

    // Mock the external call so it returns our expected response
    // This allows testing contract interactions without deploying external contracts
    vm.mock_call(external_contract, call_data, Ok(expected_response));

    // SECTION 7: Time-Dependent Behavior Testing
    // -----------------------------------------

    // Set a specific block timestamp
    // This simulates the passage of time on the blockchain
    vm.set_block_timestamp(1006);

    // Try giving another cupcake after the time restriction has passed
    // The contract should allow this since enough time has elapsed
    assert!(contract.give_cupcake_to(user).unwrap());

    // Verify the user now has two cupcakes
    // This confirms our contract correctly handles time-based restrictions
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(2)
    );
}