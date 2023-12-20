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
    #[allow(non_camel_case_types)]
    #[allow(clippy::extra_unused_lifetimes)]
    struct DynCat<'a, meow, name, change_name, aaaa>
    where
        meow: FnMut(&String) -> String,
        name: FnMut(&String) -> &'a str,
        change_name: FnMut(&mut String, String),
        aaaa: FnMut(&String, &String, &[u8]) -> &'static str,
    {
        state: &'a mut String,
        meow: Mutex<&'a mut meow>,
        name: Mutex<&'a mut name>,
        change_name: Mutex<&'a mut change_name>,
        aaaa: Mutex<&'a mut aaaa>,
    }

    #[allow(non_camel_case_types)]
    impl<
            'state,
            meow: FnMut(&String) -> String,
            name: FnMut(&String) -> &'state str,
            change_name: FnMut(&mut String, String),
            aaaa: FnMut(&String, &String, &[u8]) -> &'static str,
        > CatObj for DynCat<'state, meow, name, change_name, aaaa>
    {
        fn meow(&self) -> String {
            self.meow.lock().unwrap()(self.state)
        }
        fn name(&self) -> &str {
            self.name.lock().unwrap()(self.state)
        }
        fn change_name(&mut self, name: String) {
            self.change_name.lock().unwrap()(self.state, name)
        }
        fn aaaa<'a>(&self, a: &'a String, b: &[u8]) -> &'a str {
            self.aaaa.lock().unwrap()(self.state, a, b)
        }
    }
    let name = "Kitty";
    let mut mut_name = "Kitty".to_string();
    let mut count = 0;
    let mut meow = |_state: &String| {
        count += 1;
        format!("{}: meow {}", name, count)
    };
    let mut name_c = |#[allow(unused_variables)] state: &String| name;
    let mut change_name = |_state: &mut String, a| mut_name = a;
    let mut aaaa =
        |#[allow(unused_variables)] state: &String, a: &String, b: &[u8]| -> &str { "aaa" };
    let mut state = Default::default();
    let mut mock_cat = DynCat {
        state: &mut state,
        meow: Mutex::new(&mut meow),
        name: Mutex::new(&mut name_c),
        change_name: Mutex::new(&mut change_name),
        aaaa: Mutex::new(&mut aaaa),
    };
    assert_eq!(call_meow(&mock_cat), "Kitty: meow 1");
    assert_eq!(call_meow(&mock_cat), "Kitty: meow 2");
    assert_eq!(call_name(&mock_cat), "Kitty");
    assert_eq!(call_name2(&mock_cat), "Kitty");
    call_change_name(&mut mock_cat, "Kitty2".to_string());
    drop_cat(mock_cat);
    assert_eq!(mut_name, "Kitty2");
}
