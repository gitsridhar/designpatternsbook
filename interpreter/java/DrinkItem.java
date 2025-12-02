package interpreter.java;

public class DrinkItem implements Item {
    private String drinkName;
    private String drinkSize;

    public DrinkItem(String drinkName, String drinkSize) {
        this.drinkName = drinkName;
        this.drinkSize = drinkSize;
    }

    @Override
    public void interpret(FoodOrder foodOrder) {
        for (String item : foodOrder.getOrderItems().keySet()) {
            String details = foodOrder.getOrderItems().get(item);
            System.out.println("Item: " + item + ", Details: " + details);
        }
    }
}