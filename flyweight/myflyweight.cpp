#include <iostream>

using namespace std;

struct Fruit {
    string softness_;
    string taste_;
    string color_;

    Fruit(const string &softness,
                const string &taste,
                const string &color) :
                softness_(softness), taste_(taste), color_(color) {}

    friend ostream &operator<<(ostream &os, const Fruit &ss) {
        return os << "[ " << ss.color_ << " , " 
                          << ss.taste_ << " , " 
                          << ss.color_ << " ]";
    }
};

struct FruitInBasket {
    string baskettype_;
    string basketnumber_;

    FruitInBasket(const string &type, const string &number) :
                baskettype_(type), basketnumber_(number) {
                }
    friend ostream &operator<<(ostream &os, const FruitInBasket &us) {
        return os << "[ " << us.baskettype_ << " , " << us.basketnumber_ << " ]";
    }
};

class Flyweight {
    private:
    Fruit *fruit_;

    public:
    Flyweight(const Fruit *fruit) : fruit_(new Fruit(*fruit)) {
    }

    Flyweight(const Flyweight &other) : fruit_(new Fruit(*other.fruit_)) {
    }

    ~Flyweight() {
        delete fruit_;
    }

    Fruit *shared_state() const {
        return fruit_;
    }

    void Operation(const FruitInBasket &fruitinbasket) const {
        cout << "Flyweight: " << *fruit_ << " " << fruitinbasket << endl;
    }
};

int main() {
    return 0;
}