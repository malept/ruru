use std::convert::From;

use binding::rproc;
use types::Value;
use util;

use {AnyObject, Class, Object, VerifiedObject};

/// `Proc` (works with `Lambda` as well)
#[derive(Debug, PartialEq)]
pub struct Proc {
    value: Value,
}

impl Proc {
    /// Calls a proc with given arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{Class, Object, Proc, RString};
    ///
    /// class!(Greeter);
    ///
    /// methods!(
    ///     Greeter,
    ///     itself,
    ///
    ///     fn greet_rust_with(greeting_template: Proc) -> RString {
    ///         let name = RString::new("Rust").to_any_object();
    ///         let rendered_template = greeting_template.unwrap().call(vec![name]);
    ///
    ///         rendered_template.try_convert_to::<RString>().unwrap()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Greeter", None).define(|itself| {
    ///         itself.def_self("greet_rust_with", greet_rust_with);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Greeter
    ///   def self.greet_rust_with(greeting_template)
    ///     greeting_template.call('Rust')
    ///   end
    /// end
    ///
    /// greeting_template = -> (name) { "Hello, #{name}!" }
    ///
    /// Greeter.greet_rust_with(greeting_template) # => "Hello, Rust!"
    /// ```
    pub fn call(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);
        let result = rproc::call(self.value, argc, argv.as_ptr());

        AnyObject::from(result)
    }
}

impl From<Value> for Proc {
    fn from(value: Value) -> Self {
        Proc { value: value }
    }
}

impl Object for Proc {
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Proc {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Proc")
    }

    fn error_message() -> String {
        "Error converting to Proc".to_string()
    }
}
