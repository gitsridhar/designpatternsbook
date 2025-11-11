package factorymethod.java;

public class BananaConsumer extends Consumer {
    public Fruit getFruit() {
        return new Banana();
    }
}
