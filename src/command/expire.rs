pub(crate) struct Expire {
    key: String,
    seconds: usize,
    opts: Option<Opts>,
}

enum Opts {
    NX,
    XX,
    GT,
    LT,
}
