package proxy.java;

public class MyProxy {
    public static void main(String[] args) {
        Burger vegBurger = new VegBurger();
        Burger burgerProxy = new BurgerProxy(vegBurger);
        burgerProxy.prepare();
    }
}
