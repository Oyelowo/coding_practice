fn main() {}

mod only_generics {
    // Alternative
    // trait Add<Rhs: PartialEq + PartialOrd = Self, OutType: PartialEq + PartialOrd = Rhs> {
    trait Add<Rhs = Self, OutType = Rhs>
    where
        Rhs: PartialEq + PartialOrd,
        OutType: PartialEq + PartialOrd,
    {
        fn add(&self, rhs: Rhs) -> OutType;
    }

    #[derive(PartialEq, PartialOrd)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Add<f32> for Point {
        fn add(&self, rhs: f32) -> f32 {
            todo!()
        }
    }

    // impl Add for Point {
    impl Add<Self, i32> for Point {
        fn add(&self, rhs: Self) -> i32 {
            todo!()
        }
    }

    impl Add<i32, Self> for Point {
        fn add(&self, rhs: i32) -> Self {
            todo!()
        }
    }
}

mod combining_generics_and_associated_types {

    // trait Add<T: PartialEq + PartialOrd = Self> {
    // Alternative
    trait Add<T = Self>
    where
        T: PartialEq + PartialOrd,
    {
        type OutType;

        fn add(&self, rhs: T) -> Self::OutType;
    }

    #[derive(PartialEq, PartialOrd)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Add<f32> for Point {
        type OutType = Self;

        fn add(&self, rhs: f32) -> Self::OutType {
            todo!()
        }
    }

    // impl Add for Point {
    impl Add<Self> for Point {
        type OutType = Self;

        fn add(&self, rhs: Self) -> Self::OutType {
            todo!()
        }
    }

    impl Add<i32> for Point {
        type OutType = Self;

        fn add(&self, rhs: i32) -> Self::OutType {
            todo!()
        }
    }
}

mod associated_types {

    // When to use associated type. when u have a single implementation of the trait per type e.g `Rhs` below
    trait Add {
        type Rhs;
        type OutType;

        fn add(&self, rhs: Self::Rhs) -> Self::OutType;
    }

    struct Point {
        x: i64,
        y: i64,
    }

    impl Add for Point {
        type Rhs = Self;
        type OutType = Self;

        fn add(&self, rhs: Self::Rhs) -> Self::OutType {
            todo!()
        }
    }

    /*

    This will conflict
    impl Add for Point {
        type Rhs = i32;
        type OutType = Self;

        fn add(&self, rhs: Self::Rhs) -> Self::OutType {
            todo!()
        }
    } */
}
