#include <iostream>
#include <vector>

using namespace std;

class Dish {
    private:
        string name;
    public:
        Dish(string name) : name(name){}
        string getName() {
            return name;
        }
};

class Eating {
    public:
    virtual bool hasNextDish() = 0;
    virtual Dish *nextDish() = 0;

    virtual ~Eating() {}
};

class RestaurantEating : public Eating {
    private:
    vector<Dish> &dishes;
    int index;

    public:
    RestaurantEating(vector<Dish> &dishes) : dishes(dishes),index(0) {}

    bool hasNextDish() override {
        return index < dishes.size();
    }

    Dish *nextDish() override {
        if (hasNextDish()) {
            return &dishes[index++];
        }
        return nullptr;
    }
};

class Dinner {
    public:
    virtual Eating *createDinner() = 0;
    virtual ~Dinner() = default;
};

class WeekendDinner : public Dinner {
    private:
    vector<Dish> items;

    public:
    WeekendDinner(vector<Dish> dishes) : items(dishes) {}

    
    Eating *createDinner() override {
        return new RestaurantEating(items);
    }
};

int main() {
    vector<Dish> dishes;
    dishes.push_back(Dish("Soup"));
    dishes.push_back(Dish("Fries"));
    dishes.push_back(Dish("Sandwich"));
    dishes.push_back(Dish("Dessert"));

    WeekendDinner weekenddinner(dishes);

    Eating *eating = weekenddinner.createDinner();

    while(eating->hasNextDish()) {
        cout << eating->nextDish()->getName() << endl;
    }
    delete eating;

    return (0);
}