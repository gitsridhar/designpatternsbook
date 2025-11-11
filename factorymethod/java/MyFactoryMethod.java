package factorymethod.java;

public class MyFactoryMethod {
    public static void main(String[] args) {
        BananaConsumer bc = new BananaConsumer();
        System.out.println(bc.consume());

        AppleConsumer ac = new AppleConsumer();
        System.out.println(ac.consume());
    }
}

/*
 * javac *.java
 * cd ~/Documents/DP
 * java factorymethod.java.MyFactoryMethod
 */