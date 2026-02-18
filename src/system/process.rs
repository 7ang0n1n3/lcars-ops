use std::collections::HashSet;
use sysinfo::{Pid, System};

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    Pid,
    Name,
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
            sort_column: SortColumn::Cpu,
            sort_order: SortOrder::Descending,
            limit: 30,
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

    pub fn get_processes(&self, system: &System) -> Vec<ProcessInfo> {
        let mut procs: Vec<ProcessInfo> = system
            .processes()
            .iter()
            .map(|(pid, proc_info)| ProcessInfo {
                pid: pid.as_u32(),
                name: proc_info.name().to_string_lossy().to_string(),
                cpu_usage: proc_info.cpu_usage(),
                memory: proc_info.memory(),
            })
            .collect();

        match self.sort_column {
            SortColumn::Pid => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
            SortColumn::Name => procs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortColumn::Cpu => procs.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            SortColumn::Memory => procs.sort_by(|a, b| a.memory.cmp(&b.memory)),
        }

        if self.sort_order == SortOrder::Descending {
            procs.reverse();
        }

        procs.truncate(self.limit);
        procs
    }

    pub fn get_children(&self, parent_pid: u32, system: &System) -> Vec<ProcessInfo> {
        let mut children: Vec<ProcessInfo> = system
            .processes()
            .iter()
            .filter_map(|(pid, proc_info)| {
                if proc_info.parent() == Some(Pid::from_u32(parent_pid)) {
                    Some(ProcessInfo {
                        pid: pid.as_u32(),
                        name: proc_info.name().to_string_lossy().to_string(),
                        cpu_usage: proc_info.cpu_usage(),
                        memory: proc_info.memory(),
                    })
                } else {
                    None
                }
            })
            .collect();

        children.sort_by(|a, b| {
            b.cpu_usage
                .partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        children
    }
}
