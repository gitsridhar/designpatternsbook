#include <iostream>
#include <list>

using namespace std;

class IObserver {
    public:
    virtual ~IObserver(){};
    virtual void ReceiveNotification(const string message) = 0;
};

class ISubject {
    public:
    virtual ~ISubject(){};
    virtual void add(IObserver *observer) = 0;
    virtual void remove(IObserver *observer) = 0;
    virtual void Notify() = 0;
};

class Chef : public ISubject {
    private:
        list<IObserver *> waiters;
        string message;

    public:
        virtual ~Chef() {
        }

        void add(IObserver *observer) override {
            waiters.push_back(observer);
        }

        void remove(IObserver *observer) override {
            waiters.remove(observer);
        }

        void Notify() override {
            list<IObserver *>::iterator iterator = waiters.begin();
            CountWaiters();
            while (iterator != waiters.end()) {
                // there is no intermediary like 'message queue' in this
                // example, so the subject is simply invoking the
                // receive method of observer
                (*iterator)->ReceiveNotification(message);
                ++iterator;
            }
        }

        void CreateMessage(string message = "Empty") {
            this->message = message;
            Notify();
        }

        void CountWaiters() {
            cout << "Waiters size : " << waiters.size() << endl;
        }
};

class Waiter : public IObserver {
    public:
    Waiter(Chef &chef, string name) : chef(chef), name(name) {
        this->chef.add(this);
    }

    ~Waiter() {
    }

    void ReceiveNotification(const string message) override {
        cout << this->name << " Waiter Received message " << message << endl;
        message_ = message;
    }

    void RemoveMe() {
        chef.remove(this);
    }

    private:
    string name;
    string message_;
    Chef &chef;
};

int main() {
    Chef *chef = new Chef;

    Waiter *waiter1 = new Waiter(*chef, "Waiter 1");
    Waiter *waiter2 = new Waiter(*chef, "Waiter 2");
    Waiter *waiter3 = new Waiter(*chef, "Waiter 3");
    Waiter *waiter4;
    Waiter *waiter5;

    chef->CreateMessage("Sandwich is ready!");
    waiter3->RemoveMe();

    chef->CreateMessage("Working on the Pasta.");
    waiter4 = new Waiter(*chef, "Waiter 4");

    waiter2->RemoveMe();
    waiter5 = new Waiter(*chef, "Waiter 5");

    chef->CreateMessage("Pasta is ready.");
    waiter5->RemoveMe();

    waiter4->RemoveMe();
    waiter1->RemoveMe();

    delete waiter5;
    delete waiter4;
    delete waiter3;
    delete waiter2;
    delete waiter1;
    delete chef;

    return(0);
}


