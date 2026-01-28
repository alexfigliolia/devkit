pub trait InternalExecutable {
    fn run(&self, args: Vec<String>);
    fn help(&self);
}
