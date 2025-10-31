#include <iostream>
#include <unordered_map>
#include <vector>
#include <memory>

// Flyweight class
class TreeType {
private:
    std::string name;
    std::string color;
    std::string texture;
public:
    TreeType(const std::string& name, const std::string& color, const std::string& texture)
        : name(name), color(color), texture(texture) {}
    void display(int x, int y) const {
        std::cout << "TreeType: " << name << ", Color: " << color << ", Texture: " << texture
                  << " at (" << x << ", " << y << ")\n";
    }
};
// Flyweight Factory
class TreeFactory {
private:
    static std::unordered_map<std::string, std::shared_ptr<TreeType>> treeTypes;
public:
    static std::shared_ptr<TreeType> getTreeType(const std::string& name, const std::string& color, const std::string& texture) {
        std::string key = name + "_" + color + "_" + texture;
        if (treeTypes.find(key) == treeTypes.end()) {
            treeTypes[key] = std::make_shared<TreeType>(name, color, texture);
            std::cout << "Creating new TreeType: " << key << "\n";
        }
        return treeTypes[key];
    }
};

std::unordered_map<std::string, std::shared_ptr<TreeType>> TreeFactory::treeTypes;

// Context class containing extrinsic state
class Tree {
private:
    int x, y;
    std::shared_ptr<TreeType> type;
public:
    Tree(int x, int y, std::shared_ptr<TreeType> type) : x(x), y(y), type(type) {}
    void draw() const { type->display(x, y); }
};
// Forest to manage trees
class Forest {
private:
    std::vector<Tree> trees;
public:
    void plantTree(int x, int y, const std::string& name, const std::string& color, const std::string& texture) {
        std::shared_ptr<TreeType> type = TreeFactory::getTreeType(name, color, texture);
        trees.emplace_back(x, y, type);
    }
    void displayTrees() const {
        for (const auto& tree : trees) {
            tree.draw();
        }
    }
};
// Main function
int main() {
    Forest forest;
    forest.plantTree(1, 2, "Oak", "Green", "Rough");
    forest.plantTree(5, 10, "Oak", "Green", "Rough");
    forest.plantTree(8, 14, "Pine", "Dark Green", "Smooth");
    forest.plantTree(3, 7, "Oak", "Green", "Rough");
    std::cout << "\nDisplaying all trees:\n";
    forest.displayTrees();
    return 0;
}