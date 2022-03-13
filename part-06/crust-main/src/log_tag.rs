#[macro_export]
macro_rules! log_tag {
    () => {{
        #[allow(dead_code)]
        fn this() {}

        #[allow(dead_code)]
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // We will extract the fully qualified path name of the 'this' method and break it into components.
        let path_components: std::vec::Vec<&str> = type_name_of(this).split("::").collect();

        // Return the second last element which will be the parent of the `this` method.
        path_components[path_components.len() - 2]
    }};
}
