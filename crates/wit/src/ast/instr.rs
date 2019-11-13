pub enum Instruction<'a> {
    ArgGet(wast::Index<'a>),
}
