//@ edition: 2024

#![feature(non_lifetime_binders)]
#![expect(incomplete_features)]

fn produce() -> for<A: A<{ //~ ERROR expected trait, found type parameter `A`
    //~^ ERROR expected a type, found a trait
    //~^ ERROR late-bound type parameter not allowed on trait object types
    #[derive(Hash)]
    enum A {}
    struct A<A>; //~ ERROR the name `A` is defined multiple times
}>> Trait {} //~ ERROR cannot find trait `Trait` in this scope

fn main() {}
