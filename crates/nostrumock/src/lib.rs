use std::sync::Mutex;

#[allow(non_camel_case_types)]
enum Method {
    meow,
    name,
    change_name,
}

#[allow(non_camel_case_types)]
enum change_name {
    name(String),
}

#[allow(non_camel_case_types)]
enum Arg {
    change_name(change_name),
}

#[allow(non_camel_case_types)]
enum Ret<'a> {
    meow(String),
    name(&'a str),
    change_name(()),
}

impl<'a> Ret<'a> {
    fn meow(self) -> String {
        match self {
            Ret::meow(meow) => meow,
            Ret::name(_) => panic!("expected meow return value but got name"),
            Ret::change_name(_) => panic!("expected meow return value but got change_name"),
        }
    }
}

trait CatObj {
    #[allow(clippy::new_ret_no_self)]
    fn new(name: String) -> Cat {
        Cat { name }
    }
    fn meow(&self) -> String {
        self.__nostrum_unimplemented(Method::meow, &[]).meow()
    }
    fn name(&self) -> &str {
        unimplemented!()
    }
    fn change_name(&mut self, name: String) {
        unimplemented!()
    }
    fn aaaa<'a>(&self, a: &'a String, b: &[u8]) -> &'a str {
        unimplemented!()
    }
    fn __nostrum_unimplemented(&self, _method: Method, _args: &[Arg]) -> Ret<'_> {
        unimplemented!()
    }
}

struct Cat {
    name: String,
}

impl CatObj for Cat {
    fn meow(&self) -> String {
        format!("{}: meow", self.name)
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
}

fn call_meow<Cat: CatObj>(cat: &Cat) -> String {
    cat.meow()
}

fn call_name<Cat: CatObj>(cat: &Cat) -> &str {
    cat.name()
}

fn call_name2(cat: &impl CatObj) -> &str {
    cat.name()
}

fn call_change_name<Cat: CatObj>(cat: &mut Cat, name: String) {
    cat.change_name(name)
}

fn drop_cat<Cat: CatObj>(_cat: Cat) {}

pub struct Test;

impl CatObj for Test {
    fn new(name: String) -> Cat {
        Cat { name }
    }

    fn meow(&self) -> String {
        self.__nostrum_unimplemented(Method::meow, &[]).meow()
    }

    fn name(&self) -> &str {
        unimplemented!()
    }

    fn change_name(&mut self, _name: String) {
        unimplemented!()
    }

    fn __nostrum_unimplemented(&self, _method: Method, _args: &[Arg]) -> Ret<'_> {
        unimplemented!()
    }
}

#[test]
fn test() {
    let name = "Kitty";
    let mut mut_name = "Kitty".to_string();
    let mut count = 0;

    #[nostrumock_macros::mock(let mut mock_cat = "Kitty".to_string())]
    impl CatObj for String {
        fn meow(&self) -> String {
            count += 1;
            format!("{}: meow {}", self, count)
        }

        fn name(&self) -> &str {
            name
        }

        fn change_name(&mut self, name: String) {
            mut_name = name;
        }
    }

    assert_eq!(call_meow(&mock_cat), "Kitty: meow 1");
    assert_eq!(call_meow(&mock_cat), "Kitty: meow 2");
    assert_eq!(call_name(&mock_cat), "Kitty");
    assert_eq!(call_name2(&mock_cat), "Kitty");
    call_change_name(&mut mock_cat, "Kitty2".to_string());
    drop_cat(mock_cat);
    assert_eq!(mut_name, "Kitty2");
}
