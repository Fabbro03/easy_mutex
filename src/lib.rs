/*
  Copyright 2025 Marco Fabbroni

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// A thread-safe, clonable wrapper around `std::sync::Mutex<T>` using `Arc`.
///
/// `EasyMutex` is a convenience wrapper for safely sharing mutable access
/// across threads using an atomic reference-counted mutex. It provides a simplified
/// interface to lock and access the underlying data.
///
/// # Example
///
/// ```
/// use easy_mutex::EasyMutex;
///
/// let shared = EasyMutex::new(5);
/// let clone = shared.clone();
///
/// assert_eq!(shared.read(), 5);
/// clone.write(10);
/// assert_eq!(shared.read(), 10);
///
/// assert!(clone.write_result(2).is_ok());
///
/// let readed  = match shared.read_result() {
///     Ok(val) => {println!("Safe read: {val}"); val},
///     Err(e) => {println!("Poisoned mutex: {e}"); 0},
/// };
/// assert_eq!(readed, 2);
///
///let data: EasyMutex<String> = "hello".to_string().into();
///assert_eq!(data.read(), "hello");
/// ```
#[derive(Clone, Default, Debug)]
pub struct EasyMutex<T>(Arc<Mutex<T>>);

impl<T> EasyMutex<T> {
    /// Creates a new `EasyMutex` wrapping the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to wrap in a mutex.
    ///
    /// # Returns
    ///
    /// An `EasyMutex` instance holding the provided value.
    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }

    /// Reads the inner value by acquiring a lock, cloning it and releasing it.
    ///
    /// # Returns
    ///
    /// A clone of the inner value.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (e.g., another thread panicked while holding the lock).
    pub fn read(&self) -> T
    where
        T: Clone,
    {
        self.0.lock().unwrap().clone()
    }

    /// Writes a new value into the mutex by acquiring a lock, replacing the inner value and releasing it.
    ///
    /// # Arguments
    ///
    /// * `new_value` - The new value to be stored in the mutex.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (e.g., another thread panicked while holding the lock).
    pub fn write(&self, new_value: T) {
        *self.0.lock().unwrap() = new_value;
    }

    /// Same as [`EasyMutex::read`], but return a `Result<T, PoisonError<MutexGuard<'_, T>>>` type.
    pub fn read_result(&self) -> Result<T, PoisonError<MutexGuard<'_, T>>>
    where
        T: Clone,
    {
        self.0.lock().map(|guard| guard.clone())
    }

    /// Same as [`EasyMutex::write`], but return a `Result<(), PoisonError<MutexGuard<'_, T>>>` type.
    pub fn write_result(&self, new_value: T) -> Result<(), PoisonError<MutexGuard<'_, T>>> {
        self.0.lock().map(|mut guard| *guard = new_value)
    }
}

/// Enables `EasyMutex::from(value)` syntax.
impl<T> From<T> for EasyMutex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::EasyMutex;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn basic_read_write() {
        let m = EasyMutex::new(10);
        assert_eq!(m.read(), 10);

        m.write(20);
        assert_eq!(m.read(), 20);
    }

    #[test]
    fn test_result_read_write() {
        let data = EasyMutex::new(1);

        let val = data.read_result().unwrap();
        assert_eq!(val, 1);

        let write_result = data.write_result(2);
        assert!(write_result.is_ok());

        let val = data.read_result().unwrap();
        assert_eq!(val, 2);
    }

    #[test]
    fn clone_mutex_and_share() {
        let m = EasyMutex::new(0);
        let m2 = m.clone();

        m.write(5);
        assert_eq!(m2.read(), 5);
    }

    #[test]
    fn test_from_impl() {
        let data: EasyMutex<String> = "hello".to_string().into();
        assert_eq!(data.read(), "hello");
    }

    #[test]
    fn concurrent_modify() {
        let m = Arc::new(EasyMutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let m_clone = m.clone();
            let handle = thread::spawn(move || {
                let start = Instant::now();
                while Instant::now().duration_since(start) < Duration::from_secs(10) {
                    m_clone.write(m_clone.read() + 1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        let final_val = m.read();
        assert!(final_val >= 10000 && final_val <= 100000000);
    }
}
