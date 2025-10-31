#include <iostream>

using namespace std;

class Burger {
    public:
    virtual void Prepare() const = 0;
};

class VegBurger : public Burger {
    public:
    void Prepare() const override {
        cout << "Preparing Veggie Burger" << endl;
    }
};

class BurgerProxy : public Burger {
    private:
    VegBurger *vegburger;

    bool TastesGood() const {
        // real: interact with veg burger
        return true;
    }

    bool IsReady() const {
        // real: interact with veg burger
        return true;
    }

    public:
    BurgerProxy(VegBurger *vegburger) : vegburger(new VegBurger(*vegburger)) {
    }

    ~BurgerProxy() {
        delete vegburger;
    }

    void Prepare() const override {
        this->vegburger->Prepare();
        if (this->IsReady()) {
            if (this->TastesGood()) {
                cout << "Ready and Tasty !" << endl;
            }
        }
    }
};

void client(const Burger &burger) {
    burger.Prepare();
}

int main() {
    VegBurger *vegburger = new VegBurger();
    client(*vegburger);

    BurgerProxy *burgerproxy = new BurgerProxy(vegburger);
    client(*burgerproxy);

    delete vegburger;
    delete burgerproxy;

    return(0);
}