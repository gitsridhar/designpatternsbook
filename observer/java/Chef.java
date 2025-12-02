package observer.java;

import java.util.ArrayList;
import java.util.List;

public class Chef extends Subject {

    private List<Observer> observers = new ArrayList<>();

    public void addObserver(Observer observer) {
        observers.add(observer);
    }

    public void removeObserver(Observer observer) {
        observers.remove(observer);
    }   

    public void notifyObservers(String message) {
        for (Observer observer : observers) {
            observer.update(message);
        }
    }
    
    public void prepareDish(String dishName) {
        System.out.println("Chef is preparing: " + dishName);
        notifyObservers("Dish prepared: " + dishName);
    }
    
}
