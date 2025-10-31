#include <iostream>

using namespace std;

class Chef {
    public:
    virtual Chef *Next(Chef *chef) = 0;
    virtual string Prepare(string request) = 0;
};

class BasicChef : public Chef {
    private:
    Chef *next_chef;

    public:
    BasicChef() : next_chef(nullptr) {}

    Chef *Next(Chef *chef) override {
        this->next_chef = chef;

        return chef;
    }

    string Prepare(string request) override {
        if (this->next_chef) {
            return this->next_chef->Prepare(request);
        }

        return "";
    }
};

class CollectIngredientsChef : public BasicChef {
    public:
    string Prepare(string request) override {
        if (request == "collect")
            return "Chef: Collected the ingredients";
        else
            return BasicChef::Prepare(request);
    }
};
class BoilingChef : public BasicChef {
    public:
    string Prepare(string request) override {
        if (request == "boil")
            return "Chef: Boiled the ingredients";
        else
            return BasicChef::Prepare(request);
    }
};
class FryingChef : public BasicChef {
    public:
    string Prepare(string request) override {
        if (request == "fry")
            return "Chef: Fried the ingredients";
        else
            return BasicChef::Prepare(request);
    }
};
class MasterChef : public BasicChef {
    public:
    string Prepare(string request) override {
        if (request == "finish")
            return "Chef: Finished the dish!";
        else
            return BasicChef::Prepare(request);
    }
};

void client(Chef &chef) {
    vector<string> foodpreplist = {"collect","boil","fry","finish"};

    for (const string &foodprep: foodpreplist) {
        const string result = chef.Prepare(foodprep);
    }
}

int main() {
    CollectIngredientsChef *collectingredientchef = new CollectIngredientsChef;
    BoilingChef *boilingchef = new BoilingChef;
    FryingChef *fryingchef = new FryingChef;
    MasterChef *masterchef = new MasterChef;

    collectingredientchef->Next(boilingchef)->Next(fryingchef)->Next(masterchef);

    client(*collectingredientchef);

    delete collectingredientchef;
    delete boilingchef;
    delete fryingchef;
    delete masterchef;

    return(0);
}