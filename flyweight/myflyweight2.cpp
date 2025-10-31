#include <iostream>
#include <vector>

using namespace std;

class Fruit {
private:
    string name;
    string color; // red, green, blue
    string taste; // sweet, sour, bitter, pungent, salty, umami
    string size; // small, medium, large

public:
    Fruit(){}
    Fruit(string name, string color, string taste, string size) : 
          name(name), color(color), taste(taste), size(size) {}
};

class FruitFactory {
    private:
    static std::unordered_map<std::string, std::shared_ptr<Fruit>> fruits;

    public:
    static shared_ptr<Fruit> getFruit(const string &name, 
                                      const string &color, 
                                      const string &taste,
                                      const string &size) {
        string key = name + "_" + color + "_" + taste +
                     "_" + size;
        if (fruits.find(key) == fruits.end()) {
            fruits[key] = make_shared<Fruit>(name, color, taste, size);
        }
        return fruits[key];
    }
};

std::unordered_map<std::string, std::shared_ptr<Fruit>> FruitFactory::fruits;

class Apple: public Fruit {
    private:
    string table; // table name
    shared_ptr<Fruit> fruit;

    public:
    Apple(string table, shared_ptr<Fruit> fruit): table(table),
                                                  fruit(fruit) {
                                                   }
};

class Restaurant {
    private:
    vector<Apple> apples;

    public:
    void EatApple(const string &table, const string &name, 
                  const string &color, const string &taste,
                  const string &size) {
                    shared_ptr<Fruit> fruit = FruitFactory::getFruit(name, color,
                                                                     taste, size);
                    apples.emplace_back(table, fruit);
                  }
};

int main() {
    Restaurant restaurant;

    restaurant.EatApple("table1", "American", "red", "sweet", "medium");
    restaurant.EatApple("table1", "Swiss", "orange", "sour", "small");
    restaurant.EatApple("table2", "Brazilian", "green", "umami", "large");
    restaurant.EatApple("table3", "Japanese", "blue", "sour", "medium");

    return 0;
}