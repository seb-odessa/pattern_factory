mod pizza {
    pub enum Type {
        Cheese,
        Greek,
        Pepperoni,
        Clam,
        Veggie,
    }

    pub trait Pizza {
        fn prepare(&self);
        fn pack(&self);
    }

    pub struct CheesePizza;
    impl Pizza for CheesePizza {
        fn prepare(&self) {
            println!("Preparing Cheese pizza");
        }
        fn pack(&self) {
            println!("Packing Cheese pizza for delivery");
        }
    }

    pub struct GreekPizza;
    impl Pizza for GreekPizza {
        fn prepare(&self) {
            println!("Preparing Greek pizza");
        }
        fn pack(&self) {
            println!("Packing Greek pizza for delivery");
        }
    }

    pub struct PepperoniPizza;
    impl Pizza for PepperoniPizza {
        fn prepare(&self) {
            println!("Preparing Pepperoni pizza");
        }
        fn pack(&self) {
            println!("Packing Pepperoni pizza for delivery");
        }
    }

    pub struct ClamPizza;
    impl Pizza for ClamPizza {
        fn prepare(&self) {
            println!("Preparing Calm pizza");
        }
        fn pack(&self) {
            println!("Packing Calm pizza for delivery");
        }
    }

    pub struct VeggiePizza;
    impl Pizza for VeggiePizza {
        fn prepare(&self) {
            println!("Preparing Veggie pizza");
        }
        fn pack(&self) {
            println!("Packing Veggie pizza for delivery");
        }
    }
}

mod factory {
    use pizza::*;

    pub fn simple_factory(what : Type) -> Box<Pizza> {
        return match what {
            Type::Cheese    => Box::new(CheesePizza),
            Type::Greek     => Box::new(GreekPizza),
            Type::Pepperoni => Box::new(PepperoniPizza),
            Type::Clam      => Box::new(ClamPizza),
            Type::Veggie    => Box::new(VeggiePizza),
        }
    }

    pub fn order_pizza(what : Type) -> Box<Pizza> {
        let pizza = simple_factory(what);
        pizza.prepare();
        pizza.pack();
        return pizza;
    }
}

mod method {
    use pizza::*;
    use factory::simple_factory;

    pub trait Factory {
        fn create(&self, what : Type) -> Box<Pizza>;
    }
    pub fn order_pizza(factory : &Factory, pizza : Type) -> Box<Pizza> {
        let pizza = factory.create(pizza);
        pizza.prepare();
        pizza.pack();
        return pizza;
    }

    pub struct NewYorkFactory;
    impl Factory for NewYorkFactory {
        fn create(&self, what : Type) -> Box<Pizza> {
            println!("Create New York style Pizza!!!");
            simple_factory(what)
        }
    }

    pub struct ChicagoFactory;
    impl Factory for ChicagoFactory {
        fn create(&self, what : Type) -> Box<Pizza> {
            println!("Create Chicago style Pizza!!!");
            simple_factory(what)
        }
    }
}

use pizza::*;
fn main() {
    {
        println!("\tSimple Factory Demo");
        factory::order_pizza(Type::Cheese);
        factory::order_pizza(Type::Clam);
    }
    {
        use method::*;
        println!("\tFactory Method Demo");
        method::order_pizza(&NewYorkFactory, Type::Pepperoni);
        method::order_pizza(&ChicagoFactory, Type::Veggie);
        //factory::order_pizza(Type::Clam);
    }

}
