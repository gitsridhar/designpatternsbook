#include <iostream>

using namespace std;

class Dish { // Memento
    private:
    string state_;

    public:
    Dish(const string &state) : state_(state) {
    }
    string getState() const {
        return state_;
    }
};

class Waiter { // Originator
    private:
    string state_;

    public:
    Waiter(const string &state) : state_(state) {
        cout << "Waiter: Initial state " << state << endl;
    }

    void serve() {
        state_ = "State changed to served.";
        cout << "Waiter: State changed to served" << endl;
    }

    Dish *save() {
        cout << "Waiter: Saving the state" << endl;
        return new Dish(state_);
    }

    void restore(Dish *dish) {
        state_ = dish->getState();
        cout << "Waiter: Restore state to " << state_ << endl;
    }
};

class Chef { // Caretaker
    private:
    vector<Dish *> dishes;
    Waiter *waiter;

    public:
    Chef(Waiter *waiter) : waiter(waiter) {
    }
    ~Chef() {
        for (Dish *dish : dishes) {
            delete dish;
        }
    }

    void backup() {
        dishes.push_back(waiter->save());
        cout << "Chef: dishes backup" << endl;
    }

    void undo() {
        if (dishes.empty()) {
            cout << "Chef: No dishes" << endl;
            return;
        }

        Dish *dish = dishes.back();
        dishes.pop_back();
        cout << "Chef: Restoring the case" << endl;
        waiter->restore(dish);
        delete dish;
    }
};

int main() {
    Waiter *waiter = new Waiter("Starting");
    Chef *chef = new Chef(waiter);

    chef->backup();
    waiter->serve();

    chef->backup();
    waiter->serve();

    chef->undo();
    chef->undo();

    delete waiter;
    delete chef;

    return (0);
}