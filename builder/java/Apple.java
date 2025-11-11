package builder.java;
import java.util.Vector;

public class Apple implements Fruit {
    public Vector<String> utensils = new Vector<String>();
    @Override
    public void getUtensils() {
        for (String utensil : utensils) {
            System.out.println(utensil);
        }
    }
}
