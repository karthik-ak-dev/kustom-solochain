//! # Todo List Pallet
//!
//! A simple todo list pallet that demonstrates the essential components of
//! writing a FRAME pallet. This pallet allows users to create, complete, and delete todo items.
//!
//! ## Overview
//!
//! This pallet implements basic todo list functionality:
//! - Creating new todo items
//! - Marking todo items as complete
//! - Deleting todo items
//! - Storing todo items with their completion status
//! - Emitting events for todo operations
//! - Handling errors for invalid operations
//!
//! ## Technical Details
//!
//! ### Storage
//! - Uses a StorageMap to store todos with u32 keys
//! - Uses a StorageValue to maintain the next available todo ID
//!
//! ### Security
//! - Only the owner of a todo can modify or delete it
//! - Title length is bounded to prevent storage abuse
//! - ID counter checks for overflow
//!
//! ### Events
//! Emits events for:
//! - Todo creation
//! - Todo completion
//! - Todo deletion

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*; // This brings Vec into scope

    /// Struct representing a single todo item
    ///
    /// Properties:
    /// - title: The todo item's text (bounded to 100 bytes for storage efficiency)
    /// - completed: Boolean flag indicating if the todo is done
    /// - owner: The account that created this todo
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Todo<AccountId> {
        /// The title of the todo item, limited to 100 bytes
        pub title: BoundedVec<u8, ConstU32<100>>,
        /// Whether the todo is completed
        pub completed: bool,
        /// The account that owns this todo
        pub owner: AccountId,
    }

    /// The pallet's configuration trait
    /// This is where we declare types and constants that the pallet depends on.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The type that will be used for Event variants
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type for weight information
        type WeightInfo: WeightInfo;
    }

    /// Main storage for todos
    ///
    /// This is a mapping from todo ID (u32) to Todo struct.
    /// We use Blake2_128Concat as the hasher for good performance and security.
    #[pallet::storage]
    pub type Todos<T: Config> = StorageMap<
        _,                  // Prefix (automatically set by the system)
        Blake2_128Concat,   // Hasher
        u32,                // Key (todo ID)
        Todo<T::AccountId>, // Value (todo struct)
    >;

    /// Counter for generating unique todo IDs
    ///
    /// This value automatically starts at 0 thanks to ValueQuery
    /// and is incremented each time a new todo is created.
    #[pallet::storage]
    pub type NextTodoId<T> = StorageValue<_, u32, ValueQuery>;

    /// Events emitted by this pallet
    ///
    /// Events help external entities (like apps) track what happened in the chain.
    /// Each event includes the account that triggered it and relevant data.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emitted when a new todo is created
        /// Parameters: [todo_id, creator_account]
        TodoCreated { todo_id: u32, who: T::AccountId },
        /// Emitted when a todo is marked complete
        /// Parameters: [todo_id, completer_account]
        TodoCompleted { todo_id: u32, who: T::AccountId },
        /// Emitted when a todo is deleted
        /// Parameters: [todo_id, deleter_account]
        TodoDeleted { todo_id: u32, who: T::AccountId },
    }

    /// Custom errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Returned when trying to modify a non-existent todo
        TodoNotFound,
        /// Returned when someone tries to modify a todo they don't own
        NotTodoOwner,
        /// Returned when the todo title exceeds 100 bytes
        TitleTooLong,
        /// Returned if we've reached the maximum number of todos (u32::MAX)
        TodoIdOverflow,
    }

    /// Dispatchable functions (callable extrinsics)
    /// These are the functions that users can call to interact with the pallet
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates a new todo item
        ///
        /// Parameters:
        /// - origin: The call origin (must be signed)
        /// - title: The title of the todo item
        ///
        /// Flow:
        /// 1. Verify the caller
        /// 2. Convert title to bounded vector
        /// 3. Generate new todo ID
        /// 4. Create and store todo
        /// 5. Emit event
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn create_todo(origin: OriginFor<T>, title: Vec<u8>) -> DispatchResult {
            // Ensure the call is signed and get the signer's account
            let who = ensure_signed(origin)?;

            // Convert the title to a bounded vector, ensuring it's not too long
            let bounded_title = BoundedVec::<u8, ConstU32<100>>::try_from(title)
                .map_err(|_| Error::<T>::TitleTooLong)?;

            // Get the next todo ID and increment it
            let todo_id = NextTodoId::<T>::get();
            let next_todo_id = todo_id.checked_add(1).ok_or(Error::<T>::TodoIdOverflow)?;

            // Create the new todo item
            let todo = Todo {
                title: bounded_title,
                completed: false,
                owner: who.clone(),
            };

            // Store the todo in chain storage
            Todos::<T>::insert(todo_id, todo);
            NextTodoId::<T>::put(next_todo_id);

            // Emit the creation event
            Self::deposit_event(Event::TodoCreated { todo_id, who });

            Ok(())
        }

        /// Marks a todo as complete
        ///
        /// Parameters:
        /// - origin: The call origin (must be signed)
        /// - todo_id: The ID of the todo to complete
        ///
        /// Flow:
        /// 1. Verify the caller
        /// 2. Find and verify todo ownership
        /// 3. Update completion status
        /// 4. Emit event
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn complete_todo(origin: OriginFor<T>, todo_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // try_mutate allows us to modify storage in a closure
            Todos::<T>::try_mutate(todo_id, |maybe_todo| -> DispatchResult {
                // Get a mutable reference to the todo or return TodoNotFound
                let todo = maybe_todo.as_mut().ok_or(Error::<T>::TodoNotFound)?;

                // Verify ownership
                ensure!(todo.owner == who, Error::<T>::NotTodoOwner);

                // Mark as complete
                todo.completed = true;

                // Emit completion event
                Self::deposit_event(Event::TodoCompleted { todo_id, who });
                Ok(())
            })
        }

        /// Deletes a todo item
        ///
        /// Parameters:
        /// - origin: The call origin (must be signed)
        /// - todo_id: The ID of the todo to delete
        ///
        /// Flow:
        /// 1. Verify the caller
        /// 2. Check todo exists and verify ownership
        /// 3. Remove from storage
        /// 4. Emit event
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn delete_todo(origin: OriginFor<T>, todo_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Get the todo and verify ownership
            let todo = Todos::<T>::get(todo_id).ok_or(Error::<T>::TodoNotFound)?;
            ensure!(todo.owner == who, Error::<T>::NotTodoOwner);

            // Remove the todo from storage
            Todos::<T>::remove(todo_id);

            // Emit deletion event
            Self::deposit_event(Event::TodoDeleted { todo_id, who });

            Ok(())
        }
    }
}
