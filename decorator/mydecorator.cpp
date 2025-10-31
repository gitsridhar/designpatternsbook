#include <iostream>
#include <typeinfo>

using namespace std;

class Food {
    public:
    virtual ~Food(){}
    virtual string dip() const = 0;
};

class Strawberry : public Food {
    public:
    string dip() const override {
        return "Strawberry dip into ... something";
    }
};

class Sauce : public Food {
    protected:
    Food *food_;

    public:
    Sauce(Food *food) : food_(food) {
    }

    string dip() const override {
        return this->food_->dip();
    }
};

class ChocolateSauce : public Sauce {
    public:
    ChocolateSauce(Food *food) : Sauce(food) {
    }

    string dip() const override {
        return "Dip into Chocolate Sauce...";
    }
};

class HotSauce : public Sauce {
    public:
    HotSauce(Food *food) : Sauce(food) {
    }

    string dip() const override {
        string a = typeid(food_).name();
        return "Dip into Hot Sauce...";
    }
};

void client(Food *food) {
    cout << food->dip() << endl;
}

int main() {
    Food *strawberry = new Strawberry();
    client(strawberry);

    Food *chocolatesauce = new ChocolateSauce(strawberry);
    Food *hotsauce = new HotSauce(strawberry);

    client(hotsauce);

    delete strawberry;
    delete chocolatesauce;
    delete hotsauce;
}

