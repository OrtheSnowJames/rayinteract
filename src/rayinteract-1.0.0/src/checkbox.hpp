#ifndef CHECKBOX_HPP
#define CHECKBOX_HPP

#include "raylib.h"
#include <string>

class Checkbox {
private:
    Rectangle bounds;      // The rectangle for the checkbox
    bool isChecked;        // Whether the checkbox is checked
    Color boxColor;        // Color of the checkbox
    Color checkColor;      // Color of the check mark
    Color borderColor;     // Border color
    std::string label;     // Label text
    Color labelColor;      // Label text color
    int fontSize;          // Font size of the label

public:
    Checkbox(float x, float y, float size, const std::string& label)
        : bounds{x, y, size, size}, isChecked(false), boxColor(WHITE), checkColor(GREEN), borderColor(BLACK), label(label), labelColor(BLACK), fontSize(20) {}

    void SetColors(Color box, Color check, Color border, Color label) {
        boxColor = box;
        checkColor = check;
        borderColor = border;
        labelColor = label;
    }

    void SetFontSize(int size) {
        fontSize = size;
    }

    void Update() {
        Vector2 mousePos = GetMousePosition();
        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON) && CheckCollisionPointRec(mousePos, bounds)) {
            isChecked = !isChecked;
        }
    }

    void Draw() const {
        DrawRectangleRec(bounds, boxColor);
        DrawRectangleLinesEx(bounds, 2, borderColor);
        if (isChecked) {
            DrawRectangle(static_cast<int>(bounds.x + bounds.width * 0.25),
                          static_cast<int>(bounds.y + bounds.height * 0.25),
                          static_cast<int>(bounds.width * 0.5),
                          static_cast<int>(bounds.height * 0.5),
                          checkColor);
        }

        DrawText(label.c_str(), static_cast<int>(bounds.x + bounds.width + 10),
                 static_cast<int>(bounds.y + bounds.height / 2 - fontSize / 2),
                 fontSize, labelColor);
    }

    bool IsChecked() const {
        return isChecked;
    }
};

#endif // CHECKBOX_HPP

