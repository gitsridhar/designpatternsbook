package bridge.java;

public class PotatoFry extends Food {
    public PotatoFry(Pan pan) {
        super(pan);
    }

    @Override
    public void prepare() {
        System.out.println("Preparing Potato Fry");
        pan.cook();
    }
}
