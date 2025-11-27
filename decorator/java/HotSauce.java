package decorator.java;

public class HotSauce extends Sauce {
    public HotSauce(Food food) {
        super(food);
    }

    @Override
    public void dip() {
        super.dip();
        System.out.println("Adding hot sauce to the food");
    }
    
}
