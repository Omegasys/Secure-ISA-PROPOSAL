use crate::core::state::SystemState;

pub struct VM {
    pub id: u64,
    pub state: SystemState,
}

pub struct VMManager {
    pub vms: Vec<VM>,
}

impl VMManager {
    pub fn new() -> Self {
        Self { vms: Vec::new() }
    }

    pub fn create_vm(&mut self, id: u64, state: SystemState) {
        self.vms.push(VM { id, state });
    }

    pub fn get_vm(&mut self, id: u64) -> Option<&mut VM> {
        self.vms.iter_mut().find(|vm| vm.id == id)
    }
}
