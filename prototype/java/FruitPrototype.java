package prototype.java;

public class FruitPrototype extends Prototype {

    @Override
    public Prototype clone() {
        return new FruitPrototype();
    }
}
