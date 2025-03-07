use clap::Parser;

#[derive(Parser, Clone)]
#[clap(next_help_heading = "RUN OPTIONS")]
pub struct RunOptions {
    #[clap(short = 'T', long, default_value = "1")]
    n_threads: usize,
    #[clap(short = 'L', long)]
    pub log_path: Option<String>,
    /// Write the option configuration to stderr
    #[clap(long)]
    pub show_options: bool,
}
impl RunOptions {
    pub fn n_threads(&self) -> usize {
        if self.n_threads == 0 {
            num_cpus::get()
        } else {
            self.n_threads.min(num_cpus::get())
        }
    }
}
