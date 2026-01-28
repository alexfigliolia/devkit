trait ExternalExecutable {
    fn run(command: String, args: Vec<String>);
    fn help(&self);
}
