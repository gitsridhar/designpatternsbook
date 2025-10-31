#include <iostream>

using namespace std;

class Restaurant1;
class Restaurant2;

class Visitor {
    public:
    virtual void Drink(const Restaurant1 *restaurant) const = 0;
    virtual void Drink(const Restaurant2 *restaurant) const = 0;
};

class Restaurant {
    public:
    virtual ~Restaurant() {}
    virtual void Invite(const Visitor *visitor) const = 0;
};

class Restaurant1 : public Restaurant {
    public:
    void Invite(const Visitor *visitor) const override {
        cout << "Visitor invited for drinks in Restaurant 1" << endl;
        visitor->Drink(this);
    }

    void TakePayment(const Visitor *visitor) const {
        cout << "Restaurant 1 Take payment" << endl;
    }
};

class Restaurant2 : public Restaurant {
    public:
    void Invite(const Visitor *visitor) const override {
        cout << "Visitor invited for drinks in Restaurant 2" << endl;
        visitor->Drink(this);
    }

    string TakePayment(const Visitor *visitor) const {
        return ("Restaurant 2 Take payment");
    }
};

class Visitor1 : public Visitor {
    public:
    void Drink(const Restaurant1 *restaurant) const override {
        restaurant->TakePayment(this);
    }

    void Drink(const Restaurant2 *restaurant) const override {
        restaurant->TakePayment(this);
    }
};

class Visitor2 : public Visitor {
    public:
    void Drink(const Restaurant1 *restaurant) const override {
        restaurant->TakePayment(this);
    }

    void Drink(const Restaurant2 *restaurant) const override {
        restaurant->TakePayment(this);
    }
};

int main() {
    array<const Restaurant *, 2> restaurants = {new Restaurant1, new Restaurant2};
    Visitor1 *visitor1 = new Visitor1();
    for (const Restaurant *restaurant : restaurants) {
        restaurant->Invite(visitor1);
    }

    Visitor2 *visitor2 = new Visitor2();
    for (const Restaurant *restaurant : restaurants) {
        restaurant->Invite(visitor2);
    }

    for (const Restaurant *restaurant : restaurants) {
        delete restaurant;
    }

    delete visitor1;
    delete visitor2;

    return(0);
}