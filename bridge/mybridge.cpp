#include <iostream>

using namespace std;

class ConsumeAs {
    public:
    virtual ~ConsumeAs(){}
    virtual string Consume() const = 0;
};

class ConsumeAsJuice : public ConsumeAs {
    public:
    string Consume() const override {
        return "Juice the fruit and drink";
    }
};

class ConsumeAsJelly : public ConsumeAs {
    public:
    string Consume() const override {
        return "Cook the fruit and eat";
    }
};

class ConsumptionAbstraction {
    protected:
    ConsumeAs *consumeas_;

    public:
    ConsumptionAbstraction(ConsumeAs *consumeas) : consumeas_(consumeas) {
    }
    
    virtual ~ConsumptionAbstraction(){}

    virtual string performConsume() const {
        return (this->consumeas_->Consume());
    }
};

class ConsumptionImplementation : public ConsumptionAbstraction {
    public:
    ConsumptionImplementation(ConsumeAs *consumeas) : ConsumptionAbstraction(consumeas) {};

    string performConsume() const override {
        return this->consumeas_->Consume();
    }
};

void client(const ConsumptionAbstraction &abstraction) {
    cout << abstraction.performConsume() << endl;
}

int main() {
    ConsumeAs *consumeas = new ConsumeAsJuice;

    ConsumptionAbstraction *consumptionabstraction = new ConsumptionAbstraction(consumeas);

    client(*consumptionabstraction);

    delete consumptionabstraction;
    delete consumeas;

    consumeas = new ConsumeAsJelly;
    consumptionabstraction = new ConsumptionImplementation(consumeas);
    client(*consumptionabstraction);

    delete consumptionabstraction;
    delete consumeas;
}