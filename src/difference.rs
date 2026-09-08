enum Difference {
    OnlyInLeft {
        path: String,
        value: String,
    },
    OnlyInRight {
        path: String,
        value: String,
    },
    Mismatch {
        path: String,
        left_value: String,
        right_value: String,
    },
}
