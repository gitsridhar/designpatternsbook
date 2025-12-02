package chain.java;

public class FryingChef implements Chef {
    private Chef nextChef;

    @Override
    public Chef setNextChef(Chef chef) {
        this.nextChef = chef;
        return chef;
    }

    @Override
    public void cook(String dish) {
        System.out.println("FryingChef is frying ingredients for: " + dish);
        if (nextChef != null) {
            nextChef.cook(dish);
        } else {
            System.out.println("No chef available to cook: " + dish);
        }
    }
}
