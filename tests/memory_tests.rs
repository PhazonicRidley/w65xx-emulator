use w65xx_emulator::peripherals::memory::VirtualMemory;

// memory tests
#[test]
fn memory_index_test() {
    let mut virtual_memory = VirtualMemory::new();
    // Test indexing
    assert_eq!(virtual_memory[0], 0);

    // Test Mutation and indexing
    virtual_memory[0] = 0x10;
    assert_eq!(virtual_memory[0], 0x10);
}

#[test]
fn memory_rom_load_test() {
    // Set up
    let mut virtual_memory = VirtualMemory::new();
    let test_rom = (0x80..0xFF).collect::<Vec<u8>>();

    let expected_output = vec![vec![0; 0xFFFF - test_rom.len()], test_rom.clone()].concat();

    // Execute
    let res = virtual_memory.load_rom(test_rom.clone(), 0xFFFF - (test_rom.len() as u16));
    if let Err(e) = res {
        panic!("{}", e.get_message());
    }

    // Validate
    assert_eq!(0xFFFF, expected_output.len()); // santity check
    for i in 0..0xFFFF {
        assert_eq!(virtual_memory[i], expected_output[i as usize]);
    }
}

// Register Unit tests

//  #[test]
//  fn accumulator_test() {
//      let accumulator = Accumulator::new();
//      assert_eq!(accumulator.get_data(), 0);
//  }
