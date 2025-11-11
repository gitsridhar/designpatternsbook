package abstractfactory.java;

public class MyAbstractFactory {
    public static void main(String[] args) {
        FirstFactory ff = new FirstFactory();
        System.out.println(ff.consumeFruit());
        System.out.println(ff.consumeVegetable());

        SecondFactory sf = new SecondFactory();
        System.out.println(sf.consumeFruit());
        System.out.println(sf.consumeVegetable());
    }
}
