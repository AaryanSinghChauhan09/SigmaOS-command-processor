//! Integration tests for SigmaOS-mint-command-processor.

use sigmaos_mint_command_processor_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
