use structopt::StructOpt;

/// A tag game simulator with stupid agents
#[derive(StructOpt, Debug)]
#[structopt(name = "agent-tag")]
pub struct Input {
    /// During a run, announce that in the next visible frame, a tag will occur.
    /// Otherwise it is easy to miss it.
    #[structopt(short, long)]
    pub print_announce: bool,

    /// Adding this flag disables the output of the field (grid).
    /// For benchmarking, when printing the grid on the cli is not required.
    #[structopt(short, long)]
    pub disable_grid: bool,

    /// Number of agents
    #[structopt(short, long, default_value = "40")]
    pub agents: usize,

    /// Number of moves (tics) before the program finishes. Important for benchmarking, otherwise a simple kill ^C works too.
    #[structopt(short, long, default_value = "10")]
    pub moves: usize,

    /// Size of field (grid)
    #[structopt(short, long, default_value = "25")]
    pub size: usize,

    /// Number of ms between tics
    #[structopt(short, long, default_value = "1000")]
    pub time: u64,
}
