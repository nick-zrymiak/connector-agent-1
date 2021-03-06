pub mod arrow;
pub mod dummy;
pub mod mixed;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

/// A `Writer` is associated with a `TypeSystem` and a `PartitionWriter`.
/// `PartitionWriter` allows multiple threads write data into the buffer owned by `Writer`.
pub trait Writer<'a>: Sized {
    const DATA_ORDERS: &'static [DataOrder];
    type TypeSystem: TypeSystem;
    type PartitionWriter: PartitionWriter<'a, TypeSystem = Self::TypeSystem>;

    /// Construct the `Writer`.
    /// This allocates the memory based on the types of each columns
    /// and the number of rows.
    fn allocate(
        &mut self,
        nrow: usize,
        schema: Vec<Self::TypeSystem>,
        data_order: DataOrder,
    ) -> Result<()>;

    /// Create a bunch of partition writers, with each write `count` number of rows.
    fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter>;
    /// Return the schema of the writer.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionWriter` writes values to its own region. `PartitionWriter` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Writer`. This indicates
/// the `PartitionWriter` can never live longer than the parent.
pub trait PartitionWriter<'a>: Send {
    type TypeSystem: TypeSystem;

    /// Write a value of type T to the location (row, col). The value is unchecked against the schema.
    /// This function is unsafe due to unchecked.
    unsafe fn write<T: 'static>(&mut self, row: usize, col: usize, value: T)
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T>,
    {
        self.consume(row, col, value)
    }
    /// Write a value of type T to the location (row, col), checked version. If T mismatch with the
    /// schema, `ConnectorAgentError::UnexpectedType` will return.
    fn write_checked<T: 'static>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T>,
    {
        self.consume_checked(row, col, value)
    }
    /// Number of rows this `PartitionWriter` controls.
    fn nrows(&self) -> usize;
    /// Number of rows this `PartitionWriter` controls.
    fn ncols(&self) -> usize;
}

/// A type implemented `Consume<T>` means that it can consume a value `T` by adding it to it's own buffer.
pub trait Consume<T> {
    unsafe fn consume(&mut self, row: usize, col: usize, value: T);
    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()>;
}
