#include <iostream>
#include <map>

using namespace std;

class FoodOrder {
    public:
    string getItem(string itemname) {
        return items[itemname];
    }

    void setItem(string itemname, string value) {
        items[itemname] = value;
    }

    private:
    map<string, string> items;
};

class Item {
    public:
    virtual string interpret(FoodOrder &foodorder) = 0;
    virtual ~Item() = default;
};

class DrinkItem : public Item {
    public:
    DrinkItem(string value) : value_(value) {}
    string interpret(FoodOrder &foodorder) override {
        return foodorder.getItem(value_);
    }

    private:
    string value_;
};

class FoodItem : public Item {
    public:
    FoodItem(string value) : value_(value) {}
    string interpret(FoodOrder &foodorder) override {
        return foodorder.getItem(value_);
    }

    private:
    string value_;
};

class AllFood : public Item {
    public:
    AllFood(Item *drink, Item *food) : one(drink), two(food) {}
    string interpret(FoodOrder &foodorder) override {
        return one->interpret(foodorder) + " " + two->interpret(foodorder);
    }
    ~AllFood() override {
        delete one;
        delete two;
    }

    private:
    Item *one;
    Item *two;
};

int main() {
    FoodOrder foodorder;

    foodorder.setItem("drink", "Beer");
    foodorder.setItem("food", "Sandwich");

    Item *item = new AllFood(
        new DrinkItem("drink"), new FoodItem("food")
    );

    string result = item->interpret(foodorder);
    cout << "All of the food: " << result << endl;

    return(0);
}