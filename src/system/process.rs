use std::collections::HashSet;
use sysinfo::{Pid, System, Users};

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    Pid,
    Name,
    User,
    Cpu,
    Memory,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn toggle(&mut self) {
        *self = match self {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        };
    }
}

pub struct ProcessView {
    pub sort_column: SortColumn,
    pub sort_order: SortOrder,
    pub limit: usize,
    pub expanded_pids: HashSet<u32>,
}

impl Default for ProcessView {
    fn default() -> Self {
        Self {
            sort_column: SortColumn::Memory,
            sort_order: SortOrder::Descending,
            limit: 50,
            expanded_pids: HashSet::new(),
        }
    }
}

impl ProcessView {
    pub fn toggle_sort(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_order.toggle();
        } else {
            self.sort_column = column;
            self.sort_order = SortOrder::Descending;
        }
    }

    pub fn toggle_expanded(&mut self, pid: u32) {
        if !self.expanded_pids.remove(&pid) {
            self.expanded_pids.insert(pid);
        }
    }

    pub fn is_expanded(&self, pid: u32) -> bool {
        self.expanded_pids.contains(&pid)
    }

    pub fn get_processes(&self, system: &System, users: &Users) -> Vec<ProcessInfo> {
        let mut procs: Vec<ProcessInfo> = system
            .processes()
            .iter()
            .map(|(pid, proc_info)| {
                let user = proc_info
                    .user_id()
                    .and_then(|uid| users.get_user_by_id(uid))
                    .map(|u| u.name().to_string())
                    .unwrap_or_else(|| "?".to_string());
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: proc_info.name().to_string_lossy().to_string(),
                    user,
                    cpu_usage: proc_info.cpu_usage(),
                    memory: proc_info.memory(),
                }
            })
            .collect();

        match self.sort_column {
            SortColumn::Pid => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
            SortColumn::Name => procs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortColumn::User => procs.sort_by(|a, b| a.user.to_lowercase().cmp(&b.user.to_lowercase())),
            SortColumn::Cpu => procs.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            SortColumn::Memory => procs.sort_by(|a, b| a.memory.cmp(&b.memory)),
        }

        if self.sort_order == SortOrder::Descending {
            procs.reverse();
        }

        procs.truncate(self.limit);
        procs
    }

    pub fn get_children(&self, parent_pid: u32, system: &System, users: &Users) -> Vec<ProcessInfo> {
        let mut children: Vec<ProcessInfo> = system
            .processes()
            .iter()
            .filter_map(|(pid, proc_info)| {
                if proc_info.parent() == Some(Pid::from_u32(parent_pid)) {
                    let user = proc_info
                        .user_id()
                        .and_then(|uid| users.get_user_by_id(uid))
                        .map(|u| u.name().to_string())
                        .unwrap_or_else(|| "?".to_string());
                    Some(ProcessInfo {
                        pid: pid.as_u32(),
                        name: proc_info.name().to_string_lossy().to_string(),
                        user,
                        cpu_usage: proc_info.cpu_usage(),
                        memory: proc_info.memory(),
                    })
                } else {
                    None
                }
            })
            .collect();

        children.sort_by(|a, b| {
            b.memory.cmp(&a.memory)
        });
        children
    }
}
