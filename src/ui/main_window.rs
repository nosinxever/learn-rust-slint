// Use the slint macro to define our UI component
slint::slint! {
    // Export the MainWindow component so it can be used from Rust code
    // The component inherits from the built-in Window type
    export component MainWindow inherits Window {
        // Set the window's fixed dimensions
        width: 500px;
        height: 500px;
        
        // Set the window title that appears in the title bar
        title: "Hello World App";
        
        // Set the window icon using an image from our assets directory
        icon: @image-url("assets/app_icon.png");
        
        // Define a counter property that can be modified
        // 'in-out' allows both reading and writing the property from Rust code
        // Initialize it to 0
        in-out property<int> counter: 0;
        
        // Create a vertical layout to arrange elements top to bottom
        VerticalLayout {
            // Center all elements vertically in the window
            alignment: center;
            // Add 20px space between elements
            spacing: 20px;
            
            // Create a rectangle that serves as our button
            Rectangle {
                // Set button dimensions
                height: 40px;                
                width: 160px;
                
                // Center the button horizontally
                horizontal-stretch: 0;
                x: (parent.width - self.width) / 2;

                // Set dynamic background color based on button state
                // When pressed: darker gray (#e0e0e0)
                // When not pressed: lighter gray (#f0f0f0)
                background: ta.pressed ? #e0e0e0 : #f0f0f0;
                
                // Add rounded corners to the button
                border-radius: 10px;
                
                // Add touch interaction area for the button
                // 'ta :=' assigns an ID we can reference
                ta := TouchArea {
                    // When clicked, increment the counter
                    clicked => {
                        // 'root' refers to the MainWindow component
                        root.counter += 1;
                    }
                }
                
                // Add text label to the button
                Text {
                    // Set the button text
                    text: "Hello, World!";
                    // Set text size
                    font-size: 24px;
                    // Center the text horizontally and vertically
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
            
            // Create a text element to display the counter
            Text {
                // Combine static text with the counter value
                // The text updates automatically when counter changes
                text: "Count: " + root.counter;
                // Set text size
                font-size: 24px;
                // Center the text horizontally
                horizontal-alignment: center;
            }
        }
    }
}


// git config --global user.email "evan.1ee@outlook.com"
// git config --global user.name "nosinxever"
// git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
