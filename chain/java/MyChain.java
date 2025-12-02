package chain.java;

public class MyChain {
    public static void main(String[] args) {
        Chef basicChef = new BasicChef();
        basicChef.cook("Pasta");
        Chef collectingChef = new CollectingIngredientsChef();
        Chef boilingChef = new BoilingChef();
        Chef fryingChef = new FryingChef();
        Chef masterChef = new MasterChef();

        collectingChef.setNextChef(boilingChef).setNextChef(fryingChef)
                      .setNextChef(masterChef);
        collectingChef.cook("Gourmet Meal");
    }
}
