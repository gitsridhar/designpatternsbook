#include <iostream>

using namespace std;

class OrderFood;

class OrderPhase {
    protected:
    OrderFood *orderfood_;

    private:
    string name_;

    public:
    OrderPhase(){};
    OrderPhase(string name) {
        name_ = name;
    };

    virtual ~OrderPhase(){};

    void set_orderfood(OrderFood *orderfood) {
        orderfood_ = orderfood;
    }

    virtual void StartCooking() = 0;
    virtual void EndCooking() =0;
    virtual void DeliverToEat() = 0;
};

class OrderFood {
    private:
    OrderPhase *orderphase;
    string name;

    public:
    OrderFood(string name, OrderPhase *orderphase) : name(name), orderphase(nullptr) {
        this->SwitchOrderPhase(orderphase);
    }

    ~OrderFood() {
        delete orderphase;
    }

    void SwitchOrderPhase(OrderPhase *orderphase) {
        cout << "OrderFood - " << name << " - SwitchOrderPhase" << endl;
        if (this->orderphase != nullptr)
            delete this->orderphase;
        this->orderphase = orderphase;
        this->orderphase->set_orderfood(this);
    }

    void Start() {
        cout << "OrderFood : Start" << endl;
        this->orderphase->StartCooking();
    }

    void End() {
        cout << "OrderFood : End" << endl;
        this->orderphase->EndCooking();
    }

    void Ready() {
        cout << "OrderFood : Ready" << endl;
        this->orderphase->DeliverToEat();
    }
};

class ReadyOrderPhase : public OrderPhase {
    public:
    ReadyOrderPhase(string name) : OrderPhase(name){}

    void StartCooking() override {}

    void EndCooking() override {}

    void DeliverToEat() override {}
};

class EndOrderPhase : public OrderPhase {
    public:
    EndOrderPhase(string name) : OrderPhase(name) {};

    void StartCooking() override {}

    void EndCooking() override {
        cout << "EndOrderPhase switching to ReadyOrderPhase" << endl;
        this->orderfood_->SwitchOrderPhase(new ReadyOrderPhase("readyorderphase"));
    }

    void DeliverToEat() override {}
};

class StartOrderPhase : public OrderPhase {
    public:
    StartOrderPhase(string name) : OrderPhase(name) {};

    void StartCooking() override {
        cout << "StartOrderPhase switching to EndOrderPhase" << endl;
        this->orderfood_->SwitchOrderPhase(new EndOrderPhase("endorderphase"));  // SRI
    }

    void EndCooking() override {}

    void DeliverToEat() override {}
};

int main() {
    OrderFood *orderfood = new OrderFood("Pasta Order", new StartOrderPhase("startorderphase"));

    orderfood->Start();
    orderfood->End();
    orderfood->Ready();

    delete orderfood;

    return (0);
}