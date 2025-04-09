Raylib Interactive Elements Documentation
=========================================

This document provides an overview of the interactive elements implemented in C++ using the Raylib library. These components can be integrated into graphical applications or games for interactive UI functionality.

1. **Checkbox**
-----------------
### Description:
A simple checkbox that toggles between checked and unchecked states when clicked.

### Public Methods:
- `Checkbox(float x, float y, float size, const std::string& label)`
  - Constructor to initialize the checkbox.
  - Parameters:
    - `x`, `y`: Position of the checkbox.
    - `size`: Size of the checkbox.
    - `label`: Text label displayed next to the checkbox.

- `void SetColors(Color box, Color check, Color border, Color label)`
  - Set the colors for the checkbox and label.

- `void SetFontSize(int size)`
  - Set the font size for the label.

- `void Update()`
  - Handles input and toggles the state when clicked.

- `void Draw() const`
  - Renders the checkbox and its label.

- `bool IsChecked() const`
  - Returns the current state of the checkbox.

2. **Dropdown**
-----------------
### Description:
A dropdown menu that allows users to select one item from a list of predefined options.

### Public Methods:
- `Dropdown(float x, float y, float width, float height, const std::vector<std::string>& items)`
  - Constructor to initialize the dropdown menu.
  - Parameters:
    - `x`, `y`: Position of the dropdown.
    - `width`, `height`: Dimensions of the dropdown.
    - `items`: List of options available in the dropdown.

- `void SetColors(Color box, Color border, Color text, Color hover)`
  - Set the colors for the dropdown box, text, and hover state.

- `void SetFontSize(int size)`
  - Set the font size for the dropdown items.

- `void Update()`
  - Handles input, toggles visibility, and selects items when clicked.

- `void Draw() const`
  - Renders the dropdown and its items (if open).

- `int GetSelectedIndex() const`
  - Returns the index of the currently selected item.

- `std::string GetSelectedItem() const`
  - Returns the currently selected item as a string.

3. **TextField**
-----------------
### Description:
A text field for user input with a customizable maximum character limit.

### Public Methods:
- `TextField(float x, float y, float width, float height, int maxLength = 32)`
  - Constructor to initialize the text field.
  - Parameters:
    - `x`, `y`: Position of the text field.
    - `width`, `height`: Dimensions of the text field.
    - `maxLength`: Maximum number of characters allowed (default: 32).

- `void SetColors(Color background, Color border, Color text)`
  - Set the colors for the text field.

- `void SetFontSize(int size)`
  - Set the font size for the text.

- `void Update()`
  - Handles user input and updates the text.

- `void Draw() const`
  - Renders the text field and its current contents.

- `std::string GetText() const`
  - Returns the current text in the field.

- `int GetLastKey() const`
  - Returns the last key pressed by the user.

- `bool IsActive() const`
  - Returns whether the text field is active.

- `void Activate()`
  - Activates the text field for input.

- `void Deactivate()`
  - Deactivates the text field.

4. **Button**
-----------------
### Description:
A clickable button with a customizable label, size, and colors.

### Public Methods:
- `Button(float x, float y, float width, float height, const std::string& label)`
  - Constructor to initialize the button.
  - Parameters:
    - `x`, `y`: Position of the button.
    - `width`, `height`: Dimensions of the button.
    - `label`: Text displayed on the button.

- `void SetColors(Color background, Color border, Color text)`
  - Set the colors for the button and its label.

- `void SetFontSize(int size)`
  - Set the font size for the button label.

- `void Update()`
  - Detects if the button is clicked.

- `void Draw() const`
  - Renders the button and its label.

- `bool IsClicked() const`
  - Returns `true` if the button was clicked.

### Usage Example:
1. Initialize Raylib:
   ```cpp
   InitWindow(800, 600, "Raylib Interactive Elements");
   ```
2. Create interactive elements:
   ```cpp
   Checkbox checkbox(100, 100, 20, "Enable Option");
   Dropdown dropdown(200, 100, 150, 30, {"Option 1", "Option 2", "Option 3"});
   TextField textField(100, 200, 200, 30);
   Button button(300, 300, 100, 40, "Click Me");
   ```
3. Update and draw in the game loop:
   ```cpp
   while (!WindowShouldClose()) {
       BeginDrawing();
       ClearBackground(RAYWHITE);

       checkbox.Update();
       dropdown.Update();
       textField.Update();
       button.Update();

       checkbox.Draw();
       dropdown.Draw();
       textField.Draw();
       button.Draw();

       EndDrawing();
   }
   CloseWindow();
   ```


## Install
Install instructions are at the bottom of the [CMakeLists.](./CMakeLists.txt)