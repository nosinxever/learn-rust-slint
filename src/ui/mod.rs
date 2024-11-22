// Make the main_window module public so it can be used outside this module
pub mod main_window;

// Re-export the MainWindow component for easier access
// This allows users to write 'use ui::MainWindow' instead of 'use ui::main_window::MainWindow'
pub use main_window::MainWindow;
