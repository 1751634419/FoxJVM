mod constant_pool;
mod class_reader;
mod attribute_info;

pub mod simple_loader {
    use crate::class::*;
    use std::fs::File;
    use std::fs;
    use crate::class::simple_loader::class_reader::{ClassReader, ClassFile};

    pub enum ClassPathType {
        // Zip,
        Folder,
    }

    pub struct SimpleClassLoader {
        class_path: String,
        class_path_type: ClassPathType,
    }

    impl SimpleClassLoader {
        pub fn new(class_path: String, class_path_type: ClassPathType) -> SimpleClassLoader {
            return SimpleClassLoader {
                class_path,
                class_path_type,
            }
        }

        fn load_class_file(&self, path: String) -> Option<&Class> {
            let file_path = format!("{}{}.class", self.class_path, path.replace("/", "\\"));
            let result = fs::read(file_path);
            if result.is_err() {
                let opt : Option<&Class> = None;
                return opt;
            }

            let data = result.unwrap();
            let mut reader = ClassReader::new(data);
            let mut file = ClassFile::new(&mut reader);

            let opt : Option<&Class> = None;
            return opt;
        }
    }

    impl ClassLoader for SimpleClassLoader {
        fn load_class(&self, class_name: &str) -> Option<&Class> {
            return self.load_class_file(class_name.to_string());
        }
    }
}