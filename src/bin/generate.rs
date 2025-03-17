use clap::Parser;
use fto_solver::{moving::{sym::SymTable, turn::TurnTable}, pruning::{statesetfto2::StateSetFTO2, statesetfto2split::StateSetFTO2Split, statesetpyra::StateSetPyra, PruningTable}, state::{statecenters::StateCenters, statecorners::StateCorners, statefto2::StateFTO2, statepyra::StatePyra, symhash::SymHash}};


/// Generate Table for the FTO Solver
#[derive(Parser)]
struct Cli {
    /// The table to generate: aux, fto2, fto2split
    table: String,
    /// Disables Progress Output
    #[clap(long, short, action)]
    quiet: bool,
    /// Fills Max Depth at end
    #[clap(long, short, action)]
    filllast: bool
}

fn main() {
    let args = Cli::parse();
    match args.table.as_str() {
        "aux" => {
            <StatePyra as SymHash>::generate_table();
            <StatePyra as TurnTable>::generate_table();
        }
        "aux2" => {
            <StateCorners as SymHash>::generate_table();
            <StateCorners as TurnTable>::generate_table();
            <StateCenters as TurnTable>::generate_table();
            <StateCenters as SymTable>::generate_table();
            <StateFTO2 as SymHash>::generate_table();
            <StateFTO2 as TurnTable>::generate_table();
        }
        "pyra" => {PruningTable::<StateSetPyra>::new(!args.quiet).generate(args.filllast);}
        "fto2" => {PruningTable::<StateSetFTO2>::new(!args.quiet).generate(args.filllast);}
        "fto2split" => {PruningTable::<StateSetFTO2Split>::new(!args.quiet).generate(args.filllast);}
        _ => {}
    }
}