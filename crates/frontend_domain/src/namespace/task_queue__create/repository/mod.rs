pub mod create_task_queue;
pub mod find_namespace_by_id;
pub mod find_task_queue_by_name;

pub trait Repository:
    find_namespace_by_id::Port + find_task_queue_by_name::Port + create_task_queue::Port
{
    //
}
