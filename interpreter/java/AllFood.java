package interpreter.java;

public class AllFood implements Item {

    private Item drinkItem;
    private Item foodItem;

    public AllFood(Item drinkItem, Item foodItem) {
        this.drinkItem = drinkItem;
        this.foodItem = foodItem;
    }

    @Override
    public void interpret(FoodOrder foodOrder) {
        drinkItem.interpret(foodOrder);
        foodItem.interpret(foodOrder);
    }
}
