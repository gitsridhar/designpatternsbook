#include <iostream>
using namespace std;

class Fruit {
    public:
        Fruit() {}
        virtual ~Fruit() {}
        virtual string Consume() const = 0;
};

class Banana : public Fruit {
    public:
        string Consume() const override {
            return "Peel and Eat.";
        }
};

class Apple : public Fruit {
    public:
        string Consume() const override {
            return "Cut and Eat.";
        }
};

class Consumer {
    public:
        Consumer() {}
        virtual ~Consumer() {}
        virtual Fruit *GetFruit() const = 0;
        string Consume() const {
            Fruit *fruit = this->GetFruit();
            string result = "Method of consumption " + fruit->Consume();
            delete fruit;

            return result;
        }
};

class BananaConsumer: public Consumer {
    public:
        Fruit *GetFruit() const override {
            return new Banana();
        }
};

class AppleConsumer: public Consumer {
    public:
        Fruit *GetFruit() const override {
            return new Apple();
        }
};


// Utility method, it is common to all consumers
// specific consumer is passed in
void Usage(const Consumer &consumer) {
    cout << consumer.Consume() << endl;
}

int main() {
    Consumer *bananaconsumer = new BananaConsumer();
    Usage(*bananaconsumer);     // option 1
    bananaconsumer->Consume();  // option 2

    Consumer *appleconsumer = new AppleConsumer();
    Usage(*appleconsumer);      // option 1
    appleconsumer->Consume();   // option 2
}