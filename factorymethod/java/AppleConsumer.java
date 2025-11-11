package factorymethod.java;

public class AppleConsumer extends Consumer {
    public Fruit getFruit() {
        return new Apple();
    }
}
