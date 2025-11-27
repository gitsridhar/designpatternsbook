package bridge.java;

public class MyBridge {
    public static void main(String[] args) {
        Pan steelPan = new SteelPan();
        Food potatoFry = new PotatoFry(steelPan);
        potatoFry.prepare();
    }
}
