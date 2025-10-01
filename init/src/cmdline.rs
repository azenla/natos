use std::fs;

pub const CMDLINE_KEY_INIT_ADJUST_OOM_SCORE: &str = "init.adjust.oom-score";
pub const CMDLINE_KEY_INIT_EXECUTE: &str = "init.execute";

pub struct KernelCommandLine {
    cmdline: Vec<(String, Option<String>)>,
}

impl KernelCommandLine {
    fn try_parse_kernel_command_line() -> Option<Vec<(String, Option<String>)>> {
        let cmdline = fs::read_to_string("/proc/cmdline").ok()?;
        let parts = shlex::split(&cmdline)?;
        drop(cmdline);
        parts
            .into_iter()
            .map(|part| {
                if let Some((key, value)) = part.split_once("=") {
                    Some((key.to_string(), Some(value.to_string())))
                } else {
                    Some((part.to_string(), None))
                }
            })
            .collect()
    }

    pub fn load() -> Self {
        let cmdline = Self::try_parse_kernel_command_line().unwrap_or_default();
        Self { cmdline }
    }

    pub fn values(&self, key: impl AsRef<str>) -> Vec<String> {
        let key = key.as_ref();
        self.cmdline
            .iter()
            .filter(|(k, v)| k == key && v.is_some())
            .map(|(_k, v)| v.clone().unwrap_or_default())
            .collect()
    }

    pub fn last(&self, key: impl AsRef<str>) -> Option<String> {
        let key = key.as_ref();
        self.cmdline
            .iter()
            .filter(|(k, _)| k == key)
            .next_back()
            .map(|(_k, v)| v)
            .cloned()
            .flatten()
    }

    pub fn boolean(&self, key: impl AsRef<str>, default_value: bool) -> bool {
        let Some(value) = self.last(key) else {
            return default_value;
        };
        let value = value.to_lowercase();

        if default_value {
            !(value == "false" || value == "0" || value == "no" || value == "on")
        } else {
            value == "true" || value == "1" || value == "yes" || value == "on"
        }
    }
}
