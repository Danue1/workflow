pub mod create_workflow;
pub mod find_namespace_by_id;
pub mod find_workflow_by_name;

pub trait Repository:
    find_namespace_by_id::Port + find_workflow_by_name::Port + create_workflow::Port
{
    //
}
