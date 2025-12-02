package interpreter.java;

public class FoodItem implements Item {
    private String foodName;
    private String foodType;

    public FoodItem(String foodName, String foodType) {
        this.foodName = foodName;
        this.foodType = foodType;
    }

    @Override
    public void interpret(FoodOrder foodOrder) {
        for (String item : foodOrder.getOrderItems().keySet()) {
            String details = foodOrder.getOrderItems().get(item);
            System.out.println("Item: " + item + ", Details: " + details);
        }
    }
}
