#include <iostream>
#include <thread>
#include <chrono>

using namespace std;

class Singleton {
    private:
    static Singleton *instance;
    static mutex mutex;

    protected:
    Singleton(const string name): name(name) {
    }
    ~Singleton() {};
    string name;

    public:
    Singleton(Singleton &other) = delete;
    void operator=(const Singleton &) = delete;

    static Singleton *GetInstance(const string &name) {
        lock_guard<std::mutex> lock(mutex);
        if (instance == nullptr) {
            instance = new Singleton(name);
        }
        return instance;
    }

    string value() const {
        return name;
    }
};

Singleton *Singleton::instance(nullptr);
mutex Singleton::mutex;

void Thread1() {
    std::this_thread::sleep_for(chrono::milliseconds(1000));
    Singleton *singleton = Singleton::GetInstance("One");

    cout << singleton->value() << endl;
}

void Thread2() {
    this_thread::sleep_for(chrono::milliseconds(1000));
    Singleton *singleton = Singleton::GetInstance("Two");

    cout << singleton->value() << endl;
}

int main() {
    thread t1(Thread1);
    thread t2(Thread2);

    t1.join();
    t2.join();

    return 0;
}