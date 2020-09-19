pub mod jvm;
pub mod class;

#[cfg(test)]
mod tests {
    use super::class::simple_loader::simple_loader::*;
    use super::class::class::*;
    use super::jvm::jvm::*;

    #[test]
    fn it_works() {
        let mut loaders : Vec<Box<dyn ClassLoader>> = vec![];

        let rt_class_loader : SimpleClassLoader = SimpleClassLoader::new("D:\\JVM\\rt\\".to_string(), ClassPathType::Folder);
        loaders.push(Box::new(rt_class_loader));

        let my_class_loader : SimpleClassLoader = SimpleClassLoader::new("D:\\Rustlang\\JVM\\Test\\out\\production\\Test\\".to_string(), ClassPathType::Folder);
        loaders.push(Box::new(my_class_loader));

        let vm : VirtualMachine = VirtualMachine::new(loaders);
        vm.load_class("test/Main");
    }
}