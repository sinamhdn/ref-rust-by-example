pub trait HelloMacro {
    //! here if we want to do something like pinting the name of the type that this macro is defined on we can't do that is run time so we need to write a macro to generate the code for us
    fn hello_macro();
}
