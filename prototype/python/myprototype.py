from abc import ABC, abstractmethod

class ProtoType(ABC):
    def __init__(self, name):
        self.name = name
    def clone(self):
        pass
        
class FruitProtoType(ProtoType):
    def clone(self):
        return FruitProtoType("Cloned Fruit")
    
class VegetableProtoType(ProtoType):
    def clone(self):
        return VegetableProtoType("Cloned Vegetable")

class ProtoTypeFactory(ABC):
    prototypes = []
    def createProtoType(self):
        self.prototypes.append(FruitProtoType("fruit"))
        self.prototypes.append(VegetableProtoType("vegetable"))
    
    def getProtoTypes(self):
        return self.prototypes
        
def main():
    ptf = ProtoTypeFactory()
    ptf.createProtoType()
    
    prototypes = ptf.getProtoTypes()
    
    for prototype in prototypes:
        print(prototype.name)
        
if __name__ == "__main__":
    main()