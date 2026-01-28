use std::ffi::OsStr;
use std::process::Command;
use std::str;

pub struct Executor {}

impl Executor {
    pub fn exec<T: AsRef<OsStr>>(command: T, args: Option<&[T]>) -> String {
        let output = Executor::spawn(command, args)
            .output()
            .expect("command failed to execute");
        if output.status.success() {
            return Executor::unwrap(&output.stdout);
        }
        return Executor::unwrap(&output.stderr);
    }

    pub fn with_stdio<T: AsRef<OsStr>>(command: T, args: Option<&[T]>) {
        let mut child = Executor::spawn(command, args)
            .spawn()
            .expect("Failed to execute");
        child.wait().expect("failed to wait on child process");
    }

    pub fn spawn<T: AsRef<OsStr>>(program: T, args: Option<&[T]>) -> Command {
        let unwrapped_args = args.unwrap_or(&[]);
        let mut command = Executor::platform_command();
        command.arg(program).args(unwrapped_args);
        command
    }

    fn platform_command() -> Command {
        if cfg!(target_os = "windows") {
            return Executor::windows_command();
        } else {
            return Executor::unix_command();
        };
    }

    fn windows_command() -> Command {
        let mut child_process = Command::new("cmd");
        child_process.arg("/C");
        child_process
    }

    fn unix_command() -> Command {
        let mut child_process = Command::new("sh");
        child_process.arg("-c");
        child_process
    }

    fn unwrap(io: &Vec<u8>) -> String {
        return str::from_utf8(io)
            .expect("Invalid output")
            .trim()
            .to_string();
    }
}
