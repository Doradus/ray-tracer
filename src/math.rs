pub fn clamp<T>(value: T, min: T, max: T) -> T 
    where T : PartialOrd {
        if value < min {
            return min;
        } else if value > max {
            return max;
        } else {
            value
        }
}