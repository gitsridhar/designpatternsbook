#include <iostream>
#include <list>

using namespace std;

class Dish {
    protected: 
    Dish *parent_;

    public:
    virtual ~Dish(){}
    void SetParent(Dish *dish) {
        parent_ = dish;
    }
    Dish *GetParent() const {
        return this->parent_;
    }
    virtual void Add(Dish *dish) {}
    virtual void Remove(Dish *dish) {}

    virtual bool IsComposite() const {
        return false;
    }
    virtual string prepare() const = 0;
};

class SaltAndPepper : public Dish {
    public:
    string prepare() const override {
        return "Salt and Pepper Prepared and Served";
    }
};

class FruitSalad : public Dish {
    public:
    string prepare() const override {
        return "Fruit Salad Prepared and Served";
    }
};

class Soup : public Dish {
    public:
    string prepare() const override {
        return "Soup Prepared and Served";
    }
};

class MainDish : public Dish {
    public:
    string prepare() const override {
        return "Main Dish Prepared and Served";
    }
};

class Serving: public Dish {
    protected:
    list<Dish *> dishes;

    public:
    void Add(Dish *dish) override {
        this->dishes.push_back(dish);
        dish->SetParent(this);
    }
    void Remove(Dish *dish) override {
        this->dishes.remove(dish);
        dish->SetParent(nullptr);
    }
    bool IsComposite() const override {
        return true;
    }
    string prepare() const override {
        string result;
        for (const Dish *d: dishes) {
            if (d == dishes.back()) {
                result += d->prepare();
            } else {
                result += d->prepare() + " and ";
            }
        }
        return "Serving prepared as " + result + ".";
    }
};

void client1(Dish *dish) {
    cout << dish->prepare() << endl;
}

void client2(Dish *dish1, Dish *dish2) {
    if (dish1->IsComposite()) {
        dish1->Add(dish2);
    }
    cout << dish1->prepare() << endl;
}

int main() {
    Dish *saltandpepper = new SaltAndPepper;
    client1(saltandpepper);

    Dish *dinner = new Serving();
    Dish *appetizer = new Serving();
    Dish *soup = new Soup();
    Dish *fruitsalad = new FruitSalad();
    Dish *maindish = new MainDish();

    appetizer->Add(soup);
    appetizer->Add(fruitsalad);

    Dish *maincourse = new Serving();
    maincourse->Add(maindish);

    dinner->Add(appetizer);
    dinner->Add(maincourse);

    client1(dinner);

    client2(dinner,fruitsalad);

    delete fruitsalad;
    delete dinner;
    delete appetizer;
    delete soup;
    delete maindish;
    delete maincourse;

    return 0;
}