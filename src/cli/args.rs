use clap::Arg;

pub enum Argument {
    Flag(&'static str, Option<char>, &'static str),
    Unary(&'static str, Option<char>, &'static str, bool),
    Unbounded(&'static str, Option<char>, &'static str, bool),
}

impl From<Argument> for Arg {
    fn from(flag: Argument) -> Self {
        match flag {
            Argument::Flag(name, short_name, description) => {
                let mut arg = Arg::new(name).long(name).num_args(0).help(description);

                if let Some(short_name) = short_name {
                    arg = arg.short(short_name);
                }

                return arg;
            }
            Argument::Unary(name, short_name, description, required) => {
                let mut arg = Arg::new(name)
                    .long(name)
                    .required(required)
                    .num_args(1)
                    .help(description);

                if let Some(short_name) = short_name {
                    arg = arg.short(short_name);
                }

                return arg;
            }
            Argument::Unbounded(name, short_name, description, required) => {
                let mut arg = Arg::new(name)
                    .long(name)
                    .required(required)
                    .num_args(1)
                    .help(description);

                if let Some(short_name) = short_name {
                    arg = arg.short(short_name);
                }

                return arg;
            }
        }
    }
}
