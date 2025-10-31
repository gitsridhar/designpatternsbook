#include <iostream>

using namespace std;


// Make Food Processor a chopper

class FoodProcessor {
    public:
    virtual ~FoodProcessor() = default;

    virtual string processfood() const {
        return "Process food";
    }
};

class Chopper {
    public:
    string chop() const {
        return "Chop Food";
    }
};

class ChopperAdapter: public FoodProcessor {
    private:
    Chopper *chopper_;

    public:
    ChopperAdapter(Chopper *chopper): chopper_(chopper) {}
    string processfood() const override {
        cout << "Food Processor Adapter : " << this->chopper_->chop() << endl;
        return "Food Processor is now a Chopper!";
    }
};

void client(const FoodProcessor *foodprocessor) {
    cout << foodprocessor->processfood() << endl;
}

int main() {
    FoodProcessor *foodprocessor = new FoodProcessor();
    client(foodprocessor);
    Chopper *chopper = new Chopper();
    cout << chopper->chop() << endl;

    ChopperAdapter *chopperadapter = new ChopperAdapter(chopper);
    client(chopperadapter);

    delete chopper;
    delete foodprocessor;
    delete chopperadapter;

    return 0;
}