#include <iostream>

using namespace std;

class Fruit {
    public:
        virtual ~Fruit(){};
        virtual string Consume() const = 0;
};

class Banana: public Fruit {
    public:
        string Consume() const override {
            return "Banana: Peel and Eat";
        }
};

class Apple: public Fruit {
    public:
        string Consume() const override {
            return "Apple: Cut and Eat";
        }
};

class Vegetable {
    public:
        virtual ~Vegetable(){};
        virtual string CookAndConsume() const = 0;
        virtual string Serve() const = 0;
};

class Potato: public Vegetable {
    public:
        string CookAndConsume() const override {
            return "Potato: Fry and Eat";
        }

        string Serve() const override {
            return "Potato: In a bag";
        }
};

class Beans: public Vegetable {
    public:
        string CookAndConsume() const override {
            return "Beans: Boil and Eat";
        }

        string Serve() const override {
            return "Beans: In a bowl";
        }
};

class MyFactory {
    public:
        virtual ~MyFactory(){};
        virtual Fruit *ConsumeFruit() const = 0;
        virtual Vegetable *ConsumeVegetable() const = 0;
};

class FirstFactory: public MyFactory {
    public:
        Fruit *ConsumeFruit() const override {
            return new Banana();
        }
        Vegetable *ConsumeVegetable() const override {
            return new Potato();
        }
};

class SecondFactory: public MyFactory {
    public:
        Fruit *ConsumeFruit() const override {
            return new Apple();
        }
        Vegetable *ConsumeVegetable() const override {
            return new Beans();
        }
};

void Usage(const MyFactory &factory) {
    const Fruit *fruit = factory.ConsumeFruit();
    const Vegetable *vegetable = factory.ConsumeVegetable();

    cout << fruit->Consume() << endl;
    cout << vegetable->CookAndConsume() << endl;
    cout << vegetable->Serve() << endl;

    delete fruit;
    delete vegetable;
}

int main() {
    FirstFactory *f1 = new FirstFactory();
    Usage(*f1);
    delete f1;

    SecondFactory *f2 = new SecondFactory();
    Usage(*f2);
    delete f1;
}