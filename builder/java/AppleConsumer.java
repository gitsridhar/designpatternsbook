package builder.java;

public class AppleConsumer implements Consumer {

    private Apple fruit;

    public AppleConsumer() {
       fruit =  new Apple();
    }
    @Override
    public void Peeler() {
        fruit.utensils.add("Peeler");
    }

    @Override
    public void Knife() {
        fruit.utensils.add("Knife");
    }
    @Override
    public Fruit getFruit() {
        return fruit;
    }
}
