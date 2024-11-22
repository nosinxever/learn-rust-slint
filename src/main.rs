// Import the ui module that contains our UI components
mod ui;
// Import the MainWindow component from our ui module
use ui::MainWindow;
// Import the ComponentHandle trait which provides the run() method
use slint::ComponentHandle;

/// The main entry point of our application
fn main() {
    // Create a new instance of MainWindow
    // new() -> Creates a new instance of our window
    // unwrap() -> Extracts the window instance, panics if creation fails
    // run() -> Starts the event loop for our window
    // unwrap() -> Extracts the result, panics if running fails
    MainWindow::new().unwrap().run().unwrap();
}