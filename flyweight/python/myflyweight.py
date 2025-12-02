class TreeType:
    def __init__(self, name, color, texture):
        self.name = name
        self.color = color
        self.texture = texture

    def display(self, x, y):
        print(f"Displaying {self.name} tree of color {self.color} "
              "with texture {self.texture} at ({x}, {y})")

class TreeFactory:
    _tree_types = {}

    @classmethod
    def get_tree_type(cls, name, color, texture):
        key = (name, color, texture)
        if key not in cls._tree_types:
            cls._tree_types[key] = TreeType(name, color, texture)
        return cls._tree_types[key]

class Tree:
    def __init__(self, x, y, tree_type):
        self.x = x
        self.y = y
        self.tree_type = tree_type

    def display(self):
        self.tree_type.display(self.x, self.y)
        
class Forest:
    def __init__(self):
        self.trees = []

    def plant_tree(self, x, y, name, color, texture):
        tree_type = TreeFactory.get_tree_type(name, color, texture)
        tree = Tree(x, y, tree_type)
        self.trees.append(tree)

    def display(self):
        for tree in self.trees:
            tree.display()

# Example usage
if __name__ == "__main__":
    forest = Forest()
    forest.plant_tree(10, 20, "Oak", "Green", "Rough")
    forest.plant_tree(15, 25, "Pine", "Dark Green", "Scaly")
    forest.plant_tree(10, 20, "Oak", "Green", "Rough")  
    # Reuses the same TreeType instance
    forest.display()


    