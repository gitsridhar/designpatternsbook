package prototype.java;

public class PrototypeFactory {
    public FruitPrototype fp;
    public VegetablePrototype vp;

    public PrototypeFactory() {
        fp = new FruitPrototype();
        vp = new VegetablePrototype();
    }

    public Prototype clone1() {
        return fp;
    }

    public Prototype clone2() {
        return vp;
    }
}
