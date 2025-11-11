package abstractfactory.java;

public class SecondFactory extends Consumer {
    public String consumeFruit() {
        return new Apple().consume();
    }
    public String consumeVegetable() {
        return new Beans().consume();
    }
}
