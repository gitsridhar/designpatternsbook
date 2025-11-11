package abstractfactory.java;

public class FirstFactory extends Consumer {
    public String consumeFruit() {
        return new Banana().consume();
    }
    public String consumeVegetable() {
        return new Potato().consume();
    }
}
