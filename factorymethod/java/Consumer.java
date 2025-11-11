package factorymethod.java;

abstract class Consumer {
    public String consume() {
        return this.getFruit().consume();
    }
    public abstract Fruit getFruit();
}
