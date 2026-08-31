export class BookRepo {
    constructor(private db: any) {}

    // ONE method, ONE result shape: all books, sorted.
    async findAllBooks(sort: string) {
        // Build the query from data passed down by the service.
        const query = `SELECT id, title, author FROM books ORDER BY ${sort}`;
        const [rows] = await this.db.execute(query);
        return rows;
    }

    // SEPARATE method for one book. No optional toggle parameter.
    async findBookById(bookId: number) {
        const query = "SELECT id, title, author FROM books WHERE id = ?";
        const [rows] = await this.db.execute(query, [bookId]);
        return rows[0];
    }
}
