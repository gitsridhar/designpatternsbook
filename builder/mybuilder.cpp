//
//  main.cpp
//  DesignPatternBuilder
//
//  Created by Sridhar Venkat on 10/19/25.
//

#include <iostream>

using namespace std;

class Fruit {
public:
    vector<string> utensils_;
    
    void getUtensils() const {
        for (size_t i=0; i < utensils_.size(); i++) {
            cout << utensils_[i] << endl;
        }
    }
};

class Consumer {
public:
    virtual ~Consumer(){};
    virtual void Peeler() const = 0;
    virtual void Knife() const = 0;
};

class FruitConsumer: public Consumer {
private:
    Fruit *fruit;
    
public:
    FruitConsumer() {
        this->StartConsuming();
    }
    
    ~FruitConsumer() {
        delete fruit;
    }
    
    void StartConsuming() {
        this->fruit = new Fruit();
    }
    
    void Peeler() const override {
        this->fruit->utensils_.push_back("Peeler");
    }
    
    void Knife() const override {
        this->fruit->utensils_.push_back("Knife");
    }
    
    Fruit *GetFruit() {
        Fruit *fruit = this->fruit;
        this->StartConsuming();
        return fruit;
    }
};

class Director {
private:
    Consumer *consumer;
    
public:
    void set_consumer(Consumer *consumer) {
        this->consumer = consumer;
    }
    
    void ConsumeFruit() {
        this->consumer->Peeler();
        this->consumer->Knife();
    }
};

void Client(Director &director) {
    FruitConsumer *fruitconsumer = new FruitConsumer();
    director.set_consumer(fruitconsumer);
    director.ConsumeFruit();
    
    Fruit *fruit = fruitconsumer->GetFruit();
    fruit->getUtensils();
    delete fruit;
    
};

int main(int argc, const char * argv[]) {
    // insert code here...
    std::cout << "Hello, World!\n";
    
    Director *director = new Director();
    Client(*director);
    delete director;
    return 0;
    return EXIT_SUCCESS;
}
