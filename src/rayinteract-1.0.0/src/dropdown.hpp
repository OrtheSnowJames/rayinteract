#ifndef DROPDOWN_HPP
#define DROPDOWN_HPP

#include "raylib.h"
#include <string>
#include <vector>

class Dropdown {
private:
    Rectangle bounds;                   // The rectangle for the dropdown
    std::vector<std::string> items;     // The list of items in the dropdown
    int selectedIndex;                  // The index of the selected item
    bool isOpen;                        // Whether the dropdown is open
    Color boxColor;                     // Background color of the dropdown
    Color borderColor;                  // Border color
    Color textColor;                    // Text color
    Color hoverColor;                   // Hover background color
    int fontSize;                       // Font size of the text

public:
    Dropdown(float x, float y, float width, float height, const std::vector<std::string>& items)
        : bounds{x, y, width, height}, items(items), selectedIndex(-1), isOpen(false),
          boxColor(WHITE), borderColor(BLACK), textColor(BLACK), hoverColor(LIGHTGRAY), fontSize(20) {}

    void SetColors(Color box, Color border, Color text, Color hover) {
        boxColor = box;
        borderColor = border;
        textColor = text;
        hoverColor = hover;
    }

    void SetFontSize(int size) {
        fontSize = size;
    }

    void Update() {
        Vector2 mousePos = GetMousePosition();

        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            if (CheckCollisionPointRec(mousePos, bounds)) {
                isOpen = !isOpen;
            } else if (isOpen) {
                for (size_t i = 0; i < items.size(); ++i) {
                    Rectangle itemBounds = {bounds.x, bounds.y + bounds.height * (i + 1), bounds.width, bounds.height};
                    if (CheckCollisionPointRec(mousePos, itemBounds)) {
                        selectedIndex = static_cast<int>(i);
                        isOpen = false;
                        break;
                    }
                }
            }
        }
    }

    void Draw() const {
        DrawRectangleRec(bounds, boxColor);
        DrawRectangleLinesEx(bounds, 2, borderColor);

        // Draw selected item
        if (selectedIndex >= 0 && selectedIndex < static_cast<int>(items.size())) {
            DrawText(items[selectedIndex].c_str(), static_cast<int>(bounds.x + 5), static_cast<int>(bounds.y + (bounds.height - fontSize) / 2), fontSize, textColor);
        }

        // Draw dropdown arrow
        DrawText("v", static_cast<int>(bounds.x + bounds.width - fontSize), static_cast<int>(bounds.y + (bounds.height - fontSize) / 2), fontSize, textColor);

        // Draw dropdown items if open
        if (isOpen) {
            for (size_t i = 0; i < items.size(); ++i) {
                Rectangle itemBounds = {bounds.x, bounds.y + bounds.height * (i + 1), bounds.width, bounds.height};
                DrawRectangleRec(itemBounds, hoverColor);
                DrawRectangleLinesEx(itemBounds, 2, borderColor);
                DrawText(items[i].c_str(), static_cast<int>(itemBounds.x + 5), static_cast<int>(itemBounds.y + (itemBounds.height - fontSize) / 2), fontSize, textColor);
            }
        }
    }

    int GetSelectedIndex() const {
        return selectedIndex;
    }

    std::string GetSelectedItem() const {
        if (selectedIndex >= 0 && selectedIndex < static_cast<int>(items.size())) {
            return items[selectedIndex];
        }
        return "";
    }
};

#endif // DROPDOWN_HPP

