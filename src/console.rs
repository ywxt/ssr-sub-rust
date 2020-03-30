use std::collections::HashMap;
use std::fmt::Debug;
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        println!("\u{001b}[31merror:{}\u{001b}[0m",format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {
        println!("\u{001b}[33mwarning:{}\u{001b}[0m",format_args!($($arg)*))
    };
}

pub fn print_group(index: usize, name: &str, items: impl Iterator<Item = impl Debug>) {
    println!("{}. {}", index, name);
    items
        .enumerate()
        .for_each(|(i, item)| println!("    {}-{}. {:?}", index, i, item));
}

pub fn print_groups<T: Debug>(groups: &HashMap<impl AsRef<str>, impl AsRef<[T]>>) {
    groups
        .iter()
        .enumerate()
        .for_each(|(index, (name, group))| {
            print_group(index, name.as_ref(), group.as_ref().iter());
        });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_group() {
        #[derive(Debug)]
        struct TestGroup(String);
        let list = vec![
            TestGroup("Hello2".to_string()),
            TestGroup("Hello1".to_string()),
        ];
        print_group(1usize, "Hello", list.iter());
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_print_groups() {
        #[derive(Debug)]
        struct TestGroup<'a>(&'a str);
        let mut groups = HashMap::new();
        groups.insert("name1", vec![TestGroup("Hello1"), TestGroup("Hello2")]);
        groups.insert("name2", vec![TestGroup("Hello1"), TestGroup("Hello2")]);
        print_groups(&groups);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn test_print_error() {
        print_error!("Hello");
        print_error!("Hello:{}", "Hello");
        print_error!("Hello:{},{}", "Hello", "Hello");
    }

    #[test]
    fn test_print_warning() {
        print_warning!("Hello");
        print_warning!("Hello:{}", "Hello");
        print_warning!("Hello:{},{}", "Hello", "Hello");
    }
}
