pub trait Repository<T,K> {
    fn create(&self, instance: T) -> K;
    fn get(&self, key: K) -> T;
    fn update(&self, instance: T) -> bool;
    fn delete(&self, instance: T) -> bool;
}