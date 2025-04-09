#ifndef TEXTFIELD_HPP
#define TEXTFIELD_HPP

#include "raylib.h"
#include <string>

class TextField {
private:
    Rectangle bounds;
    std::string text;
    int maxLength;
    Color backgroundColor;
    Color borderColor;
    Color textColor;
    int fontSize;
    bool isActive;
    size_t cursorPosition;
    float cursorBlinkTimer;
    float backspaceHoldTimer;

public:
    TextField(float x, float y, float width, float height, int maxLength)
        : bounds{x, y, width, height}, text(""), maxLength(maxLength),
          backgroundColor(WHITE), borderColor(BLACK), textColor(BLACK),
          fontSize(20), isActive(false), cursorPosition(0),
          cursorBlinkTimer(0.0f), backspaceHoldTimer(0.0f) {}

    void SetColors(Color background, Color border, Color text) {
        backgroundColor = background;
        borderColor = border;
        textColor = text;
    }

    void SetFontSize(int size) {
        fontSize = size;
    }

    void Update() {
        cursorBlinkTimer += GetFrameTime();
        if (cursorBlinkTimer >= 1.0f) {
            cursorBlinkTimer = 0.0f;
        }

        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            Vector2 mousePos = GetMousePosition();
            isActive = CheckCollisionPointRec(mousePos, bounds);
        }

        if (isActive) {
            int key = GetKeyPressed();
            if (key > 0 && text.length() < static_cast<size_t>(maxLength)) {
                text.insert(cursorPosition, 1, static_cast<char>(key));
                cursorPosition++;
            }

            if (IsKeyPressed(KEY_BACKSPACE) && cursorPosition > 0) {
                text.erase(cursorPosition - 1, 1);
                cursorPosition--;
            }

            if (IsKeyDown(KEY_BACKSPACE)) {
                backspaceHoldTimer += GetFrameTime();
                if (backspaceHoldTimer > 0.5f) {
                    backspaceHoldTimer = 1.0f;
                    if (cursorPosition > 0) {
                        text.erase(cursorPosition - 1, 1);
                        cursorPosition--;
                    }
                }
            } else {
                backspaceHoldTimer = 0.0f;
            }

            if (IsKeyPressed(KEY_LEFT) && cursorPosition > 0) {
                cursorPosition--;
            }

            if (IsKeyPressed(KEY_RIGHT) && cursorPosition < text.length()) {
                cursorPosition++;
            }

            if (IsKeyPressed(KEY_HOME)) {
                cursorPosition = 0;
            }

            if (IsKeyPressed(KEY_END)) {
                cursorPosition = text.length();
            }
        }
    }

    void Draw() const {
        DrawRectangleRec(bounds, backgroundColor);
        DrawRectangleLinesEx(bounds, 2, isActive ? RED : borderColor);

        int textY = static_cast<int>(bounds.y + (bounds.height - fontSize) / 2);
        DrawText(text.c_str(), static_cast<int>(bounds.x) + 5, textY, fontSize, textColor);

        if (isActive && cursorBlinkTimer < 0.5f) {
            int textWidth = MeasureText(text.substr(0, cursorPosition).c_str(), fontSize);
            DrawLine(static_cast<int>(bounds.x) + 5 + textWidth, textY,
                     static_cast<int>(bounds.x) + 5 + textWidth, textY + fontSize, textColor);
        }
    }

    std::string GetText() const {
        return text;
    }

    void SetValue(const std::string& value) {
        text = value.substr(0, maxLength);
        cursorPosition = text.length();
    }

    bool IsActive() const {
        return isActive;
    }

    void Activate() {
        isActive = true;
        cursorPosition = text.length();
    }

    void Deactivate() {
        isActive = false;
    }

    void Clear() {
        text.clear();
        cursorPosition = 0;
    }

    void SetCursorPosition(size_t position) {
        if (position <= text.length()) {
            cursorPosition = position;
        }
    }

    size_t GetCursorPosition() const {
        return cursorPosition;
    }

    Rectangle GetBounds() const {
        return bounds;
    }

    void SetBounds(const Rectangle& newBounds) {
        bounds = newBounds;
    }

    int GetMaxLength() const {
        return maxLength;
    }

    void SetMaxLength(int newMaxLength) {
        maxLength = newMaxLength;
        if (text.length() > static_cast<size_t>(maxLength)) {
            text.resize(maxLength);
            cursorPosition = maxLength;
        }
    }

    Color GetBackgroundColor() const {
        return backgroundColor;
    }

    Color GetBorderColor() const {
        return borderColor;
    }

    Color GetTextColor() const {
        return textColor;
    }

    int GetFontSize() const {
        return fontSize;
    }

    void SetBackgroundColor(Color color) {
        backgroundColor = color;
    }

    void SetBorderColor(Color color) {
        borderColor = color;
    }

    void SetTextColor(Color color) {
        textColor = color;
    }

    bool IsEmpty() const {
        return text.empty();
    }

    bool IsFull() const {
        return text.length() >= static_cast<size_t>(maxLength);
    }

    bool IsValid() const {
        return !text.empty() && text.length() <= static_cast<size_t>(maxLength);
    }

    void Reset() {
        text.clear();
        cursorPosition = 0;
        isActive = false;
    }

    void SetBounds(float x, float y, float width, float height) {
        bounds = {x, y, width, height};
    }

    Rectangle GetBounds() const {
        return bounds;
    }
};

#endif // TEXTFIELD_HPP

