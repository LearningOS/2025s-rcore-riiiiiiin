//! Types related to task management

use super::TaskContext;
/// The syscall counter ds
#[derive(Copy, Clone)]
pub struct SyscallCount{
    // ==== BITCH ====
    // don't have much as 16, but might in the future
    counts: [Option<(usize, usize)>; 16], 
    // ==== BITCH ====
}

impl SyscallCount{
    /// initialize
    pub const fn new() -> Self{
        Self { counts : [None; 16] }
    }
    /// add a new entry when first syscall'd
    /// or increase the corresponding count by 1
    /// used on syscall()
    pub fn succeed(&mut self, syscall_id : usize) {
        for entry in self.counts.iter_mut(){
            match entry{
                Some((id, count)) => {
                    if *id == syscall_id{
                        *count += 1;
                        break;
                    }
                },
                None => {
                    *entry = Some((syscall_id, 1));
                    break;
                }
            }
            // if let Some(et) = entry{
                
            //     et.1 += 1;
            //     break;
            // }
            // if entry.0 == 0{
            //     // empty entry, meaning we have already reached the end
            //     // so just add a new entry
            //     entry.0 = syscall_id;
            //     entry.1 = 1;
            //     break;
            // }
            // else if entry.0 == syscall_id{
            //     entry.1 += 1;
            //     break;
            // }
        }
    }
    /// get the syscall count
    /// used in sys_trace
    pub fn get(&self, syscall_id : usize) -> usize{
        for entry in self.counts.iter(){
            match entry{
                Some((id, count)) =>{
                    if *id == syscall_id{
                        return *count;
                    }
                },
                None => { break; }
            }
        }
        0
    }
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The syscall count
    pub syscall_count : SyscallCount,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
