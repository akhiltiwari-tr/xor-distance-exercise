extern crate rand;

pub struct FoodDeliverySystem {
    addresses: Vec<u64>,
}

impl FoodDeliverySystem {
    pub fn new(addresses: Vec<u64>) -> Self {
        Self { addresses }
    }

    /// Return specified count of closest farms to the provided `position`.
    ///
    /// The closest farms are ordered from the closest to the n-th closest, where `n` is the count.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise_impl;
    ///
    /// use xor_distance_exercise_impl::FoodDeliverySystem;
    ///
    /// let delivery_system: FoodDeliverySystem<u64> = FoodDeliverySystem::new(vec![
    ///     0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
    /// ]);
    ///
    /// let position = 10;
    /// let count = 10;
    ///
    /// let closest_farms = delivery_system.closest_farms(position, count);
    /// ```
    pub fn closest_farms(&self, position: u64, count: usize) -> Vec<u64> {
        let mut sorted_farms = self.addresses.clone();
        sorted_farms.sort_by_key(|address| *address ^ position);
        sorted_farms.truncate(count);
        sorted_farms
    }

    /// Return a `Some(position)` such that `self.closest(position)` equals closest_farms and return
    /// None in case such a `position` does not exists.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise_impl;
    ///
    /// use xor_distance_exercise_impl::FoodDeliverySystem;
    ///
    /// let delivery_system: FoodDeliverySystem<u64> = FoodDeliverySystem::new(vec![
    ///     0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
    /// ]);
    ///
    /// let position = 200;
    /// let count = 10;
    ///
    /// // Get closest farms and reversed guess of possible customer's `position`.
    /// let closest_farms = delivery_system.closest_farms(position, count);
    /// let position_guess = delivery_system.reverse_closest_farms(&closest_farms).unwrap();
    ///
    /// // Check that both `position` and `position_guess` produce the same result.
    /// assert_eq!(closest_farms, delivery_system.closest_farms(position_guess, count));
    /// ```
    pub fn reverse_closest_farms(&self, closest_farms: &[u64]) -> Option<u64> {
        // TODO: This is the part of an exercise you should implement.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::FoodDeliverySystem;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use rand::{self, Rng};

    #[test]
    fn closest_farms() {
        let delivery_system: FoodDeliverySystem = FoodDeliverySystem::new(vec![
            0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
        ]);

        let result = delivery_system.closest_farms(10, 10);
        let expected = vec![8, 12, 2, 0, 1, 6, 4, 18, 19, 22];

        assert_eq!(expected, result);
    }

    #[test]
    fn reverse_closest_farms() {
        let delivery_system: FoodDeliverySystem = FoodDeliverySystem::new(vec![
            0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
        ]);

        let position = 200;
        let count = 10;

        // Get closest farms and reversed guess of possible customer's `position`.
        let closest_farms = delivery_system.closest_farms(position, count);
        let position_guess = delivery_system
            .reverse_closest_farms(&closest_farms)
            .expect("The FoodDeliverySystem::reverse_closest_farms() should return a Some(position), but None returned instead!");

        // Check that both `position` and `position_guess` produce the same result.
        assert_eq!(
            closest_farms,
            delivery_system.closest_farms(position_guess, count)
        );
    }

    #[test]
    fn reverse_closest_farms_random_position() {
        // Get 2000 random numbers.
        let mut rng = rand::thread_rng();
        let points: Vec<u64> = rng.sample_iter(&Standard).take(2000).collect();

        let delivery_system = FoodDeliverySystem::new(points);

        for _ in 0..100 {
            let position = rng.gen();
            let closest_points = delivery_system.closest_farms(position, 10);
            let guess_pos = delivery_system
                .reverse_closest_farms(&closest_points)
                .expect("The FoodDeliverySystem::reverse_closest_farms() should return a Some(position), but None returned instead!");

            assert_eq!(closest_points, delivery_system.closest_farms(guess_pos, 10));
        }
    }

    #[test]
    fn reverse_closest_farms_random_set() {
        // Get 2000 random numbers.
        let mut rng = rand::thread_rng();
        let points: Vec<u64> = rng.sample_iter(&Standard).take(200).collect();

        let delivery_system = FoodDeliverySystem::new(points.clone());

        // Try hundred random closest points collections.
        for _ in 0..100 {
            let closest_points: Vec<u64> = points
                .iter()
                // Returns `Vec<&u64>` and thus we need to map it to `Vec<u64>`.
                .choose_multiple(&mut rng, 10)
                .iter()
                .map(|&&x| x)
                .collect();

            // Most of the time the generated closest points will be invalid, as they are selected
            // randomly and required relations/inequalities are not satisfied.
            if let Some(guess_pos) = delivery_system.reverse_closest_farms(&closest_points) {
                assert_eq!(closest_points, delivery_system.closest_farms(guess_pos, 10));
            }
        }
    }

}

