pub mod jvm {
    use crate::class::class::*;

    pub struct VirtualMachine {
        class_loaders: Vec<Box<dyn ClassLoader>>
    }

    impl VirtualMachine {
        pub fn new(class_loaders: Vec<Box<dyn ClassLoader>>) -> VirtualMachine {
            return VirtualMachine {
                class_loaders,
            }
        }

        pub fn load_class(&self, class_name: &str) -> Option<&Class> {
            let len = self.class_loaders.len();
            for n in 0..len {
                let opt = self.class_loaders[n].load_class(class_name);
                if let Some(_) = opt {
                    return opt;
                }
            }
            return None;
        }
    }
}