pub trait Category {
    type Object;

    type Morphism: Fn(Self::Object) -> Self::Object;
}
