#ifndef BUTTON_HPP
#define BUTTON_HPP

#include "raylib.h"
#include <string>

class Button {
private:
    Rectangle bounds;      // The rectangle for the button
    std::string label;     // The label of the button
    Color backgroundColor; // Background color of the button
    Color borderColor;     // Border color of the button
    Color textColor;       // Color of the text
    int fontSize;          // Font size of the text
    bool isHovered;        // Whether the button is being hovered over
    bool isPressed;        // Whether the button is being pressed

public:
    Button(float x, float y, float width, float height, const std::string& label)
        : bounds{x, y, width, height}, label(label), backgroundColor(LIGHTGRAY), borderColor(DARKGRAY), textColor(BLACK), fontSize(20), isHovered(false), isPressed(false) {}

    void SetColors(Color background, Color border, Color text) {
        backgroundColor = background;
        borderColor = border;
        textColor = text;
    }

    void SetFontSize(int size) {
        fontSize = size;
    }

    void Update() {
        Vector2 mousePos = GetMousePosition();
        isHovered = CheckCollisionPointRec(mousePos, bounds);
        isPressed = isHovered && IsMouseButtonDown(MOUSE_LEFT_BUTTON);
    }

    void Draw() const {
        DrawRectangleRec(bounds, isPressed ? DARKGRAY : (isHovered ? GRAY : backgroundColor));
        DrawRectangleLinesEx(bounds, 2, borderColor);
        int textWidth = MeasureText(label.c_str(), fontSize);
        DrawText(label.c_str(), static_cast<int>(bounds.x + (bounds.width - textWidth) / 2), static_cast<int>(bounds.y + (bounds.height - fontSize) / 2), fontSize, textColor);
    }

    bool IsClicked() const {
        return isHovered && IsMouseButtonPressed(MOUSE_LEFT_BUTTON);
    }

    bool IsPressed() const {
        return isPressed;
    }
};

#endif // BUTTON_HPP

