pub trait Database {
    type Connection;

    type Transaction;

    type Error;

    fn connect(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Connection, Self::Error>> + Send;

    fn transaction(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Transaction, Self::Error>> + Send;

    fn commit(
        &self,
        transaction: Self::Transaction,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    fn rollback(
        &self,
        transaction: Self::Transaction,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}
