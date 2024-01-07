#[macro_export]
macro_rules! flag {
    ($name:expr, $short:expr, $help:expr) => {
        Arg::new($name)
            .long($name)
            .short($short)
            .help($help)
            .action(clap::ArgAction::SetTrue)
            .num_args(0)
    };
    ($name:expr, $help:expr) => {
        Arg::new($name)
            .long($name)
            .help($help)
            .action(clap::ArgAction::SetTrue)
            .num_args(0)
    };
}

#[macro_export]
macro_rules! unary {
    ($name:expr, $short:expr, $help:expr) => {
        Arg::new($name)
            .long($name)
            .short($short)
            .help($help)
            .num_args(1)
    };
    ($name:expr, $help:expr) => {
        Arg::new($name).long($name).help($help).num_args(1)
    };
}

#[macro_export]
macro_rules! unbounded {
    ($name:expr, $short:expr, $help:expr) => {
        Arg::new($name)
            .long($name)
            .short($short)
            .help($help)
            .num_args(1..)
    };
    ($name:expr, $help:expr) => {
        Arg::new($name).long($name).help($help).num_args(1..)
    };
}
