#include <iostream>

using namespace std;

class Pizza {
    public:
    void TemplateMethod() const {
        this->prepareIngredients();
        this->kneadDough();
        this->restDough();
        this->spreadDough();
        this->requiredOperation();
        this->Hook1();
        this->Hook2();
        this->Hook3();
        this->placeInOwen();
        this->cook();
    }

    protected:
    void kneadDough() const {
        cout << "Knead dough" << endl;
    }
    void restDough() const {
        cout << "Rest dough" << endl;
    }
    void spreadDough() const {
        cout << "Spread dough" << endl;
    }
    void placeInOwen() const {
        cout << "Place dough in owen" << endl;
    }
    void cook() const {
        cout << "Cook Pizza" << endl;
    }

    virtual void prepareIngredients() const = 0;
    virtual void requiredOperation() const = 0;
    virtual void Hook1() const {};
    virtual void Hook2() const {};
    virtual void Hook3() const {};
};

class CheesePizza : public Pizza {
    protected:
    void prepareIngredients() const override {
        cout << "Prepare Cheese and Dough and Sauce only" << endl;
    }
    void requiredOperation() const override {
        cout << "Perform required opration of grating cheese" << endl;
    }
};

class PepperoniPizza : public Pizza {
    protected:
    void prepareIngredients() const override {
        cout << "Prepare Cheese and Dough and Sauce only" << endl;
    }
    void requiredOperation() const override {
        cout << "Perform required opration of grating cheese" << endl;
    }
    void Hook1() const override {
        cout << "Get pepperoni pieces from box" << endl;
    }
    void Hook2() const override {
        cout << "Count pepperoni pieces according to pizza size" << endl;
    }
    void Hook3() const override {
        cout << "Place pepperoni pieces in symmetric order on the pizza" << endl;
    }
};

int main() {
    CheesePizza *cheesePizza = new CheesePizza();
    cheesePizza->TemplateMethod();

    PepperoniPizza *pepperoniPizza = new PepperoniPizza();
    pepperoniPizza->TemplateMethod();

    delete cheesePizza;
    delete pepperoniPizza;
}