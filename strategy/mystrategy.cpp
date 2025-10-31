#include <iostream>

using namespace std;

class Strategy {
    public:
    virtual ~Strategy() = default;

    virtual string BoilBeforeCooking(string_view data) const = 0;
};

class FoodPreperation {
    private:
    unique_ptr<Strategy> strategy_;

    public:
    explicit FoodPreperation(unique_ptr<Strategy> &&strategy = {}) :
    strategy_(std::move(strategy)) {
    }

    void set_strategy(unique_ptr<Strategy> &&strategy) {
        strategy_ = std::move(strategy);
    }

    void cookSomething() const {
        if (strategy_) {
            string result = strategy_->BoilBeforeCooking("hundred degrees");
        } else {
            cout << "Food Preperation: Strategy is not set" << endl;
        }
    }
};

class FirstStrategy : public Strategy {
    public:
    string BoilBeforeCooking(string_view data) const override {
        string result(data);
        cout << "First Strategy : Boil in open pan" << endl;
        return result;
    }
};

class SecondStrategy : public Strategy {
    public:
    string BoilBeforeCooking(string_view data) const override {
        string result(data);
        cout << "Second Strategy : Boil in closed pan" << endl;
        return result;
    }
};

int main() {
    FoodPreperation fp(std::make_unique<FirstStrategy>());
    fp.cookSomething();

    fp.set_strategy(std::make_unique<SecondStrategy>());
    fp.cookSomething();

    return (0);
}