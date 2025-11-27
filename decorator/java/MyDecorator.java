package decorator.java;

public class MyDecorator {
    public static void main(String[] args) {
        Food strawberryWithHotSauce = new HotSauce(new Strawberry());
        strawberryWithHotSauce.dip();

        System.out.println("-----");

        Food strawberryWithChocolateAndHotSauce = new ChocolateSauce(new HotSauce(new Strawberry()));
        strawberryWithChocolateAndHotSauce.dip();
    }
}
