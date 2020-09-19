pub mod simple_loader;

pub mod class {
    pub struct Class {

    }

    pub trait ClassLoader {
        fn load_class(&self, class_name: &str) -> Option<&Class>;
    }
}