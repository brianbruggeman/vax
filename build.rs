//! Creates features based on operating system

use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Platforms
        win: { target_os = "windows" },

        // Windows fails to run the spinner and spinners packages
        // at runtime, so remove them for now on the windows os.
        spin: { not(win) },
    }
}
