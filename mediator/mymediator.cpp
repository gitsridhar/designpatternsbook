#include <iostream>

using namespace std;

class Chef;

class Waiter {
    public:
    virtual void Inform(Chef *chef, string message) const = 0;
};

class Chef {
    protected:
    Waiter *waiter;

    public:
    Chef(Waiter *waiter = nullptr) : waiter(waiter) {
    }
    void workwithwaiter(Waiter *waiter) {
        this->waiter = waiter;
    }
};

class SoupChef : public Chef {
    public:
    void Prepare() {
        string msg = "Soup Chef Preparing Soup";
        cout << msg << endl;
        this->waiter->Inform(this, "1");
    }
    void Decorate() {
        string msg = "Soup Chef Decorating Soup";
        cout << msg << endl;
        this->waiter->Inform(this, "2");
    }
};

class SandwichChef : public Chef {
    public:
    void GrillBread() {
        string msg = "Sandwich Chef Grilling the Bread";
        cout << msg << endl;
        this->waiter->Inform(this, "3");
    }
    void Assemble() {
        string msg = "Sandwich Chef Assembling the Dish";
        cout << msg << endl;
        this->waiter->Inform(this, "4");
    }
};

class OurWaiter : public Waiter {
    private:
    SoupChef *soupchef;
    SandwichChef *sandwichchef;

    public:
    OurWaiter(SoupChef *soupchef, SandwichChef *sandwichchef) :
              soupchef(soupchef), sandwichchef(sandwichchef) {
                this->soupchef->workwithwaiter(this);
                this->sandwichchef->workwithwaiter(this);
              }
    void Inform(Chef *chef, string msg) const override {
        if (msg == "1")
            sandwichchef->GrillBread();
        if (msg == "4") {
            soupchef->Decorate();
            sandwichchef->GrillBread();
        }
    }
};

void client() {
    SoupChef *soupchef = new SoupChef;
    SandwichChef *sandwichchef = new SandwichChef;

    OurWaiter *ourwaiter = new OurWaiter(soupchef, sandwichchef);
    soupchef->Prepare();
    soupchef->Decorate();

    sandwichchef->GrillBread();
    sandwichchef->Assemble();

    delete soupchef;
    delete sandwichchef;
    delete ourwaiter;
}

int main() {
    client();

    return(0);
}
