#ifndef TEXTFIELD_HPP
#define TEXTFIELD_HPP

#include "raylib.h"
#include <string>

class TextField {
private:
    Rectangle bounds;      // The rectangle for the text field
    std::string text;      // The current text in the field
    int maxLength;         // Maximum length of the text
    Color backgroundColor; // Background color of the field
    Color borderColor;     // Border color of the field
    Color textColor;       // Color of the text
    int fontSize;          // Font size of the text
    bool isActive;         // Whether the text field is active
    int lastKey;           // Last key pressed

public:
    TextField(float x, float y, float width, float height, int maxLength = 32)
        : bounds{x, y, width, height}, text(""), maxLength(maxLength), backgroundColor(WHITE), borderColor(BLACK), textColor(BLACK), fontSize(20), isActive(false), lastKey(0) {}

    void SetColors(Color background, Color border, Color text) {
        backgroundColor = background;
        borderColor = border;
        textColor = text;
    }

    void SetFontSize(int size) {
        fontSize = size;
    }

    void Update() {
        if (isActive) {
            int key = GetKeyPressed();
            if (key > 0) {
                lastKey = key;
                if ((key >= 32 && key <= 126) && text.length() < static_cast<size_t>(maxLength)) {
                    text += static_cast<char>(key);
                } else if (key == KEY_BACKSPACE && !text.empty()) {
                    text.pop_back();
                }
            }
        }

        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            Vector2 mousePos = GetMousePosition();
            isActive = CheckCollisionPointRec(mousePos, bounds);
        }
    }

    void Draw() const {
        DrawRectangleRec(bounds, backgroundColor);
        DrawRectangleLinesEx(bounds, 2, borderColor);
        DrawText(text.c_str(), static_cast<int>(bounds.x) + 5, static_cast<int>(bounds.y) + (fontSize / 2), fontSize, textColor);
        if (isActive) {
            DrawRectangleLinesEx(bounds, 2, RED);
        }
    }

    std::string GetText() const {
        return text;
    }

    int GetLastKey() const {
        return lastKey;
    }

    bool IsActive() const {
        return isActive;
    }

    void Activate() {
        isActive = true;
    }

    void Deactivate() {
        isActive = false;
    }
};

#endif // TEXTFIELD_HPP

