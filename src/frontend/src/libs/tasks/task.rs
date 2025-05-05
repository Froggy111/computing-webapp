/// Create definitions for a Task class
/// Inputs:
/// struct_name: The name of the struct to generate
/// fn_type_name: The name of the FnMut type to generate
/// fn_trait: The FnMut trait
/// (arg_name:ident: arg_type:ty): Argument names and types for the FnMut and execute
/// method. For example: (a: i32, b: i32)
/// return_type:ty : The return type of the FnMut and execute method
#[macro_export]
macro_rules! define_task {
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
    name: &'a str,
}

impl<'a> std::fmt::Debug for $struct_name<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!($struct_name))
            .field("task", &"<FnMut Closure>")
            .field("name", &self.name)
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
            name
        }
    }

    #[inline]
    pub fn execute(&mut self, $($arg_name: $arg_type),*) -> $return_type {
        // Note: Using stringify!($struct_name) provides the struct name at compile time
        leptos::logging::log!("Executing {} '{}'", stringify!($struct_name), self.name);
        (self.task)($($arg_name),*)
    }

    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }
}
    };
}
