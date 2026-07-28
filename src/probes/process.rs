use std::io;

use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind, Users, get_current_pid};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessRecord {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeIdentitySnapshot {
    pub user: Option<String>,
    pub process_ancestry: Vec<ProcessRecord>,
}

pub trait RuntimeIdentityInspector {
    /// Observes the current OS account and process ancestry in one bounded
    /// local snapshot. No command shell or network access may be used.
    ///
    /// # Errors
    ///
    /// Returns an error when the current process is absent from the OS
    /// snapshot or when the platform does not expose a current process ID.
    fn inspect(&self, max_depth: usize) -> io::Result<RuntimeIdentitySnapshot>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemRuntimeIdentityInspector;

impl RuntimeIdentityInspector for SystemRuntimeIdentityInspector {
    fn inspect(&self, max_depth: usize) -> io::Result<RuntimeIdentitySnapshot> {
        // Process names and parent IDs are part of the base process snapshot.
        // Request only user IDs in addition to that base data: command lines,
        // environment blocks, working directories, roots, and executable paths
        // stay disabled.
        let process_refresh = ProcessRefreshKind::nothing()
            .with_user(UpdateKind::OnlyIfNotSet)
            .without_tasks();
        let system =
            System::new_with_specifics(RefreshKind::nothing().with_processes(process_refresh));
        let mut pid = get_current_pid().map_err(io::Error::other)?;
        let current = system.process(pid).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "current process is absent from the OS process snapshot",
            )
        })?;

        let user = current.user_id().and_then(|user_id| {
            let users = Users::new_with_refreshed_list();
            users
                .get_user_by_id(user_id)
                .map(|user| user.name().to_owned())
        });

        let mut process_ancestry = Vec::new();
        for _ in 0..max_depth {
            let Some(process) = system.process(pid) else {
                break;
            };
            let parent_pid = process.parent().map_or(0, sysinfo::Pid::as_u32);
            process_ancestry.push(ProcessRecord {
                pid: pid.as_u32(),
                parent_pid,
                name: process.name().to_string_lossy().into_owned(),
            });
            if parent_pid == 0 || parent_pid == pid.as_u32() {
                break;
            }
            pid = sysinfo::Pid::from_u32(parent_pid);
        }

        Ok(RuntimeIdentitySnapshot {
            user,
            process_ancestry,
        })
    }
}
