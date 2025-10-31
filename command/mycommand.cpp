#include <iostream>

using namespace std;

class Action {
    public:
    virtual ~Action(){}
    virtual void DoIt() const = 0;
};

class Peel : public Action {
    private:
    string vegetable;

    public:
    explicit Peel(string vegetable) : vegetable(vegetable) {
    }

    void DoIt() const override {
        cout << "Peeling the vegetable " << this->vegetable << endl;
    }
};

class Customer {
    public:
    void OrderFood(const string &a) {
    }
    void Pay(const string &a) {
    }
};

class InteractWithCustomer: public Action {
    private:
    Customer *customer;

    string a;

    public:
    InteractWithCustomer(Customer *customer, string a) : 
        customer(customer), a(a) {
        }
    void DoIt() const override {
        this->customer->OrderFood(this->a);
        this->customer->Pay(this->a);
    }
};

class Waiter {
    private:
    Action *action1;

    Action *action2;

    public:
    ~Waiter() {
        delete action1;
        delete action2;
    }

    void DoAction1(Action *action) {
        this->action1 = action;
    }
    void DoAction2(Action *action) {
        this->action2 = action;
    }

    void Perform() {
        if (this->action1) {
            this->action1->DoIt();
        }
        if (this->action2) {
            this->action2->DoIt();
        }
    }
};

int main() {
    Waiter *waiter = new Waiter();
    waiter->DoAction1(new Peel("Potato"));

    Customer *customer = new Customer();
    waiter->DoAction2(new InteractWithCustomer(customer, "How Can I Help You!"));
    waiter->Perform();

    delete waiter;
    delete customer;

    return (0);
}