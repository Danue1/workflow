pub mod create_namespace;
pub mod find_namespace_by_name;

pub trait Repository: find_namespace_by_name::Port + create_namespace::Port {
    //
}
