#include <iostream>

using namespace std;

class ColdFood {
    public:
    string WashRinse() const {
        return "Cold Food: Wash and rinse";
    }
    string Wrap() const {
        return "Wrap in a box or bag";
    }
    string Freeze() const {
        return "Freeze in a refrigerator";
    }
};

class HotFood {
    public:
    string Unwrap() const {
        return "Take the food out of bag";
    }
    string Clean() const {
        return "Clean the food";
    }
    string Cook() const {
        return "Cook in the heat";
    }
};

class Restaurant {
    protected:
    ColdFood *coldfood;
    HotFood *hotfood;

    public:
    Restaurant(ColdFood *coldfood = nullptr,
               HotFood *hotfood = nullptr) {
                this->coldfood = coldfood ?: new ColdFood;
                this->hotfood = hotfood ?: new HotFood;
               }
    ~Restaurant() {
        delete coldfood;
        delete hotfood;
    }
    string Operation() {
        string result = "Restaurant Opens..\n";
        result += coldfood->WashRinse() + "\n";
        result += hotfood->Unwrap() + "\n";

        return result;
    }
};

void client(Restaurant *restaurant) {
    cout << restaurant->Operation();
}

int main() {
    ColdFood *coldfood = new ColdFood;
    HotFood *hotfood = new HotFood;

    Restaurant *restaurant = new Restaurant(coldfood, hotfood);
    client(restaurant);

    return 0;
}