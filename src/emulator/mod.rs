pub mod component;

use crate::hardware::Hardware;

pub struct Emulator {
    hardware: Hardware,
    components: Vec<Box<dyn component::Component>>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            hardware: Hardware::new(),
            components: Vec::new(),
        }
    }

    pub fn register_component(&mut self, component: Box<dyn component::Component>) -> &mut Self {
        self.components.push(component);
        self
    }

    pub fn init(&mut self) -> &mut Self {
        for component in &mut self.components {
            component.init(&mut self.hardware);
        }
        self
    }

    pub fn run(&mut self) {
        loop {
            if self.hardware.power_on == false {
                break;
            }
            for component in &mut self.components {
                component.clock(&mut self.hardware);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;

    impl component::Component for TestComponent {
        fn init(&mut self, hardware: &mut Hardware) {
            hardware.pc = 0x100;
        }

        fn clock(&mut self, hardware: &mut Hardware) {
            hardware.pc += 1;
            hardware.power_on = false;
        }
    }

    #[test]
    fn test_emulator_new() {
        let emulator = Emulator::new();
        assert_eq!(emulator.hardware.pc, 0);
    }

    #[test]
    fn test_emulator_register_component() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        assert_eq!(emulator.components.len(), 1);
    }

    #[test]
    fn test_emulator_init() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        emulator.init();
        assert_eq!(emulator.hardware.pc, 0x100);
    }

    #[test]
    fn test_emulator_run() {
        let mut emulator = Emulator::new();
        emulator.register_component(Box::new(TestComponent));
        emulator.init();
        emulator.run();
        assert_eq!(emulator.hardware.pc, 0x101);
    }
}
