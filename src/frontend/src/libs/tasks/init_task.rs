/// Create definitions for an InitTask class
/// Inputs:
/// struct_name: The name of the struct to generate
/// fn_type_name: The name of the FnMut type to generate
/// fn_trait: The FnMut trait. The function MUST return a bool!
/// (arg_name:ident: arg_type:ty): Argument names and types for the FnMut and execute
/// method. For example: (a: i32, b: i32)
/// return_type:ty : The return type of the FnMut and execute method
#[macro_export]
macro_rules! define_init_task {
    (
        $struct_name:ident,
        $fn_type_name:ident,
        $fn_trait:ident,
        ($($arg_name:ident: $arg_type:ty),*),
        $return_type:ty
    ) => {

pub type $fn_type_name = Box<dyn $fn_trait>;

// Task struct
pub struct $struct_name<'a> {
    task: Box<dyn $fn_trait>,
    initialised: std::cell::Cell<bool>,

    name: &'a str,
    attempts: std::cell::Cell<i32>
}

impl<'a> std::fmt::Debug for $struct_name<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!($struct_name))
            .field("task", &"<FnMut Closure>")
            .field("initialised", &self.initialised.get())
            .field("name", &self.name)
            .field("attempts", &self.attempts)
            .finish()
    }
}

impl<'a> std::fmt::Display for $struct_name<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} '{}'", stringify!($struct_name), self.name)
    }
}

// Implement associated functions (like `new` and `execute`)
impl<'a> $struct_name<'a> {
    /// Creates a new task.
    pub fn new<F>(task: F, name: &'a str) -> Self
    where
        F: $fn_trait
    {
        Self {
            task: Box::new(task),
            initialised: std::cell::Cell::new(false),

            name,
            attempts: std::cell::Cell::new(0),
        }
    }

    pub fn execute_if_uninitialised(&mut self, $($arg_name: $arg_type),*) -> bool {
        if !self.initialised.get() {
            leptos::logging::log!("Executing {} '{}'", stringify!($struct_name), self.name);
            *self.initialised.get_mut() = (self.task)($($arg_name),*);
            *self.attempts.get_mut() += 1;
            if self.initialised.get() {
                leptos::logging::log!(
                    "{} '{}' initialised successfully. Attempts: {}",
                    stringify!($struct_name),
                    self.name,
                    self.attempts.get()
                );
            } else {
                leptos::logging::error!(
                    "{} '{}' initialisation failed. Attempts: {}",
                    stringify!($struct_name),
                    self.name,
                    self.attempts.get()
                );
            }
        }
        self.initialised.get()
    }

    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }
}
    };
}
