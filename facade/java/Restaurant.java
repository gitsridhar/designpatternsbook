package facade.java;

public class Restaurant {
    ColdFood coldFood;
    HotFood hotFood;

    Restaurant(ColdFood coldFood, HotFood hotFood) {
        this.coldFood = coldFood;
        this.hotFood = hotFood;
    }

    String operation() {
        String result = "";
        result += coldFood.washAndRinse() + "\n";
        result += coldFood.wrap() + "\n";
        result += coldFood.freeze() + "\n";
        result += hotFood.unwrap() + "\n";
        result += hotFood.clean() + "\n";
        result += hotFood.cook() + "\n";
        return result;
    }
}
