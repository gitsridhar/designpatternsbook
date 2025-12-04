protocol Food {
    func prepare()
}

protocol Pan {
    func cook()
}

class PotatoFry: Food {
    var pan = SteelPan()

    init(pan: SteelPan) {
        self.pan = pan
    }

    func prepare() {
        print("PotatoFry. Prepare")
        self.pan.cook()
    }
}

class SteelPan: Pan {
    func cook() {
        print("SteelPan. Cook")
    }
}

@main
struct MyBridge {
    static func main() {
        let pan = SteelPan()
        let food = PotatoFry(pan: pan)

        food.prepare()
    }
}

