package decorator.java;

public class ChocolateSauce extends Sauce {
    public ChocolateSauce(Food food) {
        super(food);
    }

    @Override
    public void dip() {
        super.dip();
        System.out.println("Adding chocolate sauce to the food");
    }
    
}
