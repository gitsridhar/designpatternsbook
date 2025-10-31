#include <iostream>

using namespace std;

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

class ChopperAdapter : public FoodProcessor, public Chopper {
    public:
    ChopperAdapter(){};

    string processfood() const override {
        cout << "Food Processor is now a Chopper!" << endl;
        return chop();
    }
};

void client(FoodProcessor *foodprocessor) {
    cout << foodprocessor->processfood() << endl;
}

int main() {
    FoodProcessor *foodprocessor = new FoodProcessor();
    client(foodprocessor);

    Chopper *chopper = new Chopper();
    chopper->chop();

    ChopperAdapter *chopperadapter = new ChopperAdapter();
    client(chopperadapter);

    delete foodprocessor;
    delete chopper;
    delete chopperadapter;
}