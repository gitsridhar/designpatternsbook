#include <iostream>

using namespace std;

class Prototype {
    protected:
    string prototypename;

    public:
    Prototype() {}
    Prototype(string name) : prototypename(name) {
    }
    virtual ~Prototype() {}
    virtual Prototype *Clone() const = 0;
};

class FruitPrototype: public Prototype {
    public:
        FruitPrototype(string name): Prototype(name) {
            cout << "Creating Fruit Prototype" << endl;
        }
        Prototype *Clone() const override {
            cout << "Cloning Fruit Prototype" << endl;
            return new FruitPrototype(*this);
        }
};

class VegetablePrototype: public Prototype {
    public:
        VegetablePrototype(string name): Prototype(name) {
            cout << "Creating Vegetable Prototype" << endl;
        }
        Prototype *Clone() const override {
            cout << "Cloning Vegetable Prototype" << endl;
            return new VegetablePrototype(*this);
        }
};

class PrototypeFactory {
    private:
    unordered_map<int, Prototype *, hash<int>> prototypes_;

    public:
    PrototypeFactory() {
        prototypes_[0] = new FruitPrototype("Fruit");
        prototypes_[1] = new VegetablePrototype("Vegetable");
    }
    ~PrototypeFactory() {
        delete prototypes_[0];
        delete prototypes_[1];
    }

    Prototype *CreatePrototype(int type) {
        return prototypes_[type]->Clone();
    }
};

void Client(PrototypeFactory &prototype_factory) {
    Prototype *prototype = prototype_factory.CreatePrototype(0);

    delete prototype;

    prototype = prototype_factory.CreatePrototype(1);

    delete prototype;
}

int main() {
    PrototypeFactory *prototypefactory = new PrototypeFactory();
    Client(*prototypefactory);
    delete prototypefactory;

    return 0;
}