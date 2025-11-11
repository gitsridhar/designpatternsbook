package prototype.java;

public class VegetablePrototype extends Prototype {

    @Override
    public Prototype clone() {
        return new VegetablePrototype();
    }
}
