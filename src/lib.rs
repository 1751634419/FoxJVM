pub mod env;
pub mod class;
pub mod int;

#[cfg(test)]
mod tests {
    use super::class::simple_loader::simple_loader::*;
    use super::class::*;
    use super::env::env::*;
    use crate::env::basic_env_elements::OperandStack;

    #[test]
    fn it_works() {
        let mut loaders : Vec<Box<dyn ClassLoader>> = vec![];

        let rt_class_loader : SimpleClassLoader = SimpleClassLoader::new("D:\\JVM\\rt\\".to_string(), ClassPathType::Folder);
        loaders.push(Box::new(rt_class_loader));

        let my_class_loader : SimpleClassLoader = SimpleClassLoader::new("D:\\Rustlang\\JVM\\Test\\out\\production\\Test\\".to_string(), ClassPathType::Folder);
        loaders.push(Box::new(my_class_loader));

        let env : Environment = Environment::new(loaders);
        env.load_class("test/Main");
        // env.load_class("java/lang/class");
    }
}